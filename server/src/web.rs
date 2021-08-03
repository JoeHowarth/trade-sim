use std::{
    result::Result,
    collections::HashMap,
    iter::FromIterator,
};
use types::prelude::*;
use tokio::sync::{watch, RwLock};
use types::State;
use tracing::{info};
use serde::{Serialize};
use vec_map::VecMap;
use futures::future::{AbortHandle, Abortable, Aborted};
use tokio::task::JoinHandle;
use trade_sim_grpc::{ModelReply, ModelRequest};
use tonic::{Request, Response, Status};
use std::borrow::BorrowMut;
use tonic::transport::Server;

type ModelHandle = Arc<RwLock<VecMap<Model>>>;

pub fn start_model_worker(
    mut state: watch::Receiver<State>,
    model_map: ModelHandle,
) -> (AbortHandle, JoinHandle<Result<(), Aborted>>) {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let model_worker = Abortable::new(async move {
        info!("Starting model worker task");
        while state.changed().await.is_ok() {
            info!("Detected model change. Updating model map");
            let model = state_to_model(&state.borrow());
            let mut write = model_map.write().await;
            write.insert(model.tick as usize, model.clone());
        }
    }, abort_registration);
    (abort_handle, tokio::task::spawn(model_worker))
}


#[derive(Debug, Default)]
struct ModelServerImpl {
    pub tick: Arc<Mutex<usize>>,
}

#[tonic::async_trait]
impl trade_sim_grpc::model_server_server::ModelServer for ModelServerImpl {
    async fn get_model(&self, request: Request<ModelRequest>) -> Result<Response<ModelReply>, Status> {
        let tick = request.into_inner().tick;
        *self.tick.lock().unwrap() = tick.clone() as usize;
        info!("tick {}", tick);
        Ok(Response::new(ModelReply {
            tick,
        }))
    }
}

pub async fn server(state: watch::Receiver<State>) -> Result<(), anyhow::Error> {
    let service = trade_sim_grpc::model_server_server::ModelServerServer::new(ModelServerImpl {
        tick: Arc::new(Mutex::new(12))
    });

    let model_map = Arc::new(RwLock::new(VecMap::with_capacity(100)));
    let (mw_abort, mw_join) = start_model_worker(state.clone(), model_map.clone());

    info!("Starting server");
    Server::builder()
        .add_service(service)
        .serve("127.0.0.1:3030".parse()?)
        .await?;
    info!("Server thread returning");
    // mw_join.await;
    // mw_abort.abort();


    mw_abort.abort();
    Ok(())
}

async fn handler(tick: usize, model_map: ModelHandle) {
    match model_map.read().await.get(tick) {
        Some(model) => {
            info!("Sending model at tick {}", tick);
        }
        None => {}
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

