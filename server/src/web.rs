use std::{
    collections::HashMap,
    iter::FromIterator,
};
use types::prelude::*;
use std::sync::RwLock;
use types::State;
use tracing::{info};
use serde::{Serialize};
use futures::future::{AbortHandle, Abortable};
use rouille::{Response, Request, router};
use tokio::sync::{mpsc};

type ModelHandle = Arc<RwLock<Vec<Model>>>;

pub fn start_model_worker(
    mut state: mpsc::UnboundedReceiver<State>,
    model_map: ModelHandle,
    state_list: Arc<RwLock<Vec<State>>>,
) -> AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let model_worker = Abortable::new(async move {
        while let Some(val) = state.recv().await {
            let model = state_to_model(&val);
            state_list.write().unwrap().push(val);
            let mut write = model_map.write().expect("couldn't get model map lock");
            write.insert(model.tick.clone() as usize, model);
        }
    }, abort_registration);
    info!("Starting model worker task");
    tokio::task::spawn(model_worker);
    abort_handle
}

pub async fn server(state: mpsc::UnboundedReceiver<State>) -> anyhow::Result<()> {
    let model_map = Arc::new(RwLock::new(Vec::with_capacity(100)));
    let state_list = Arc::new(RwLock::new(Vec::with_capacity(100)));
    let model_worker_handle = start_model_worker(state, model_map.clone(), state_list.clone());

    let server_handle = tokio::task::spawn_blocking(move || {
        let state_list = state_list.clone();
        let model_map = model_map.clone();
        rouille::Server::new("127.0.0.1:3030", move |request| {
            info!("Received request: {:?}", &request);
            rouille::router!(request,
                (GET) (/state) => {
                    model_handler(request, 0, model_map.clone())
                },
                (GET) (/state/{tick: usize}) => {
                    model_handler(request, tick, model_map.clone())
                },
                (GET) (/rgraph) => {
                    let state_list = state_list.read().unwrap();
                    match state_list.last() {
                        Some(state) => Response::json(&state_to_rgraph(state)),
                        None => {
                            error!("Expected non-empty state_list");
                            Response::text("Expected non-empty state list")
                            .with_status_code(500)
                        }
                    }
                },
                _ => {
                    info!("Unrecognized path {}", request.url());
                    Response::empty_404()
            })
                .with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Headers", "*")
        })
            .unwrap().run();
    });
    server_handle.await?;
    model_worker_handle.abort();
    Ok(())
}

fn model_handler(_req: &Request, tick: usize, model_map: ModelHandle) -> Response {
    match &model_map.read().expect("couldn't get model map lock").get(tick) {
        Some(model) => {
            info!("Sending model at tick {}", tick);
            Response::json(model)
        }
        None => {
            warn!("Model at tick {} not found", tick);
            Response::empty_404()
        }
    }
}

fn state_to_model(state: &State) -> Model {
    Model {
        tick: state.tick.0,
        nodes: state.nodes.iter().map(|(city, links, market_info, _pos)| {
            MNode {
                id: city.name,
                markets: {
                    let mut m: HashMap<Ustr, MarketInfo> = HashMap::new();
                    m.insert(ustr("Grain"), MarketInfo {
                        supply: market_info.supply,
                        consumption: market_info.consumption,
                        production: market_info.consumption,
                        price: market_info.current_price().0,
                    });
                    m
                },
                links: links.0.iter()
                    .map(|to| to.city.name)
                    .collect(),
            }
        }).collect(),
        edges: state.nodes.iter().flat_map(|(city, links, _market_info, _pos)| {
            links.0.iter()
                .map(|to| MEdge {
                    nodes: vec![city.name, to.city.name]
                })
                .collect::<Vec<_>>()
        }).collect(),
        agents: state.agents.iter().map(|(agent, pos, money, cargo)| {
            MAgent {
                id: agent.name,
                cargo: cargo.good.name,
                location: pos.city().expect("only node agent positions implemented").city.name,
                money: money.0,
            }
        }).collect(),
    }
}

fn state_to_rgraph(state: &State) -> RGraph {
    let m: HashMap<NodeId, RNode> = HashMap::from_iter(
        state.nodes.iter()
            .map(|(city, _links, _market_info, pos)| {
                (city.name,
                 RNode {
                     x: pos.0.x as i32,
                     y: pos.0.y as i32,
                     id: city.name,
                     radius: 1.0,
                 })
            }));
    RGraph {
        nodes: m.values().cloned().collect(),
        edges: state.nodes.iter()
            .flat_map(|(city, links, _market_info, _pos)| {
                links.0.iter()
                    .map(|to| REdge {
                        nodes: (m[&city.name], m[&to.city.name])
                    }).collect::<Vec<_>>()
            }).collect(),
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Model {
    tick: u64,
    nodes: Vec<MNode>,
    edges: Vec<MEdge>,
    agents: Vec<MAgent>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MarketInfo {
    pub supply: f64,
    pub consumption: f64,
    pub production: f64,
    pub price: f64,
}

pub type NodeId = Ustr;
pub type AgentId = Ustr;
pub type Good = Ustr;

#[derive(Serialize, Debug, Clone)]
pub struct MNode {
    pub id: NodeId,
    pub markets: HashMap<Good, MarketInfo>,
    pub links: Vec<NodeId>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MAgent {
    id: AgentId,
    cargo: Good,
    location: NodeId,
    money: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct MEdge {
    pub nodes: Vec<NodeId>,
}

#[derive(Serialize, Debug, Clone)]
pub struct RGraph {
    pub nodes: Vec<RNode>,
    pub edges: Vec<REdge>,
}

#[derive(Serialize, Clone, Debug, Copy)]
pub struct RNode {
    x: i32,
    y: i32,
    id: NodeId,
    radius: f32,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct REdge {
    pub nodes: (RNode, RNode),
}

