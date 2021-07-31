use warp::{Filter, Rejection, Reply};
use std::{
    result::Result,
    collections::HashMap,
    iter::FromIterator,
};
use types::prelude::*;
use tokio::sync::{watch, RwLock};
use warp::http::StatusCode;
use types::State;
use tracing::{info};
use serde::{Serialize};
use vec_map::VecMap;
use futures::future::{AbortHandle, Abortable};

type ModelHandle = Arc<RwLock<VecMap<Model>>>;

pub fn start_model_worker(
    mut state: watch::Receiver<State>,
    model_map: ModelHandle,
) -> AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let model_worker = Abortable::new(async move {
        while state.changed().await.is_ok() {
            let model = state_to_model(&state.borrow());
            let mut write = model_map.write().await;
            write.insert(model.tick as usize, model.clone());
        }
    }, abort_registration);
    tokio::task::spawn(model_worker);
    abort_handle
}

pub async fn server(state: watch::Receiver<State>) {
    let cors = warp::cors()
        .allow_any_origin();
    let model_map = Arc::new(RwLock::new(VecMap::with_capacity(100)));
    let model_worker_handle = start_model_worker(state.clone(), model_map.clone());


    let state_route = warp::path!("state" / usize)
        .and(warp::any().map(move || model_map.clone()))
        .and_then(handler)
        .with(cors.clone());
    let rgraph_route = warp::path("rgraph")
        .and(warp::any().map(move || state.clone()))
        .and_then(rgraph_handler)
        .with(cors);

    warp::serve(
        state_route
            .or(rgraph_route)
            .or(warp::any()
                .map(|| Ok(StatusCode::NOT_FOUND))
                .with(warp::cors().allow_any_origin())))
        .run(([127, 0, 0, 1], 3030)).await;
    model_worker_handle.abort();
}

async fn handler(tick: usize, model_map: ModelHandle) -> Result<impl Reply, Rejection> {
    match model_map.read().await.get(tick) {
        Some(model) => {
            info!("Sending model at tick {}", tick);
            Ok(warp::reply::json(&model))
        }
        None => {
            Err(warp::reject::not_found())
        }
    }
}

async fn rgraph_handler(state: watch::Receiver<State>) -> Result<impl Reply, Rejection> {
    let state = state.borrow();
    info!("RGraph is: {:?}", state);
    let model = state_to_rgraph(&state);
    Ok(warp::reply::json(&model))
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

