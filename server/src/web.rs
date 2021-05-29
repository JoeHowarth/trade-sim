use warp::{Filter, Rejection, Reply};
use std::{
    result::Result,
    collections::HashMap,
    iter::FromIterator
};
use types::prelude::*;
use tokio::sync::watch;
use warp::http::StatusCode;
use types::State;
use tracing::{info};
use serde::{Serialize};

pub async fn server(state: watch::Receiver<State>) {
    let cors = warp::cors()
        .allow_any_origin();
    let other_state = state.clone();

    let state_route = warp::path("state")
        .and(warp::any().map(move || state.clone()))
        .and_then(handler)
        .with(cors.clone());
    let rgraph_route = warp::path("rgraph")
        .and(warp::any().map(move || other_state.clone()))
        .and_then(rgraph_handler)
        .with(cors);

    warp::serve(
        state_route
            .or(rgraph_route)
            .or(warp::any()
                .map(|| Ok(StatusCode::NOT_FOUND))
                .with(warp::cors().allow_any_origin())))
        .run(([127, 0, 0, 1], 3030)).await;
}

async fn handler(state: watch::Receiver<State>) -> Result<impl Reply, Rejection> {
    let state = state.borrow();
    info!("State is: {:?}", state);
    let model = state_to_model(&state);
    info!("Model:{}", serde_json::to_string_pretty(&model).unwrap());
    Ok(warp::reply::json(&model))
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
                id: &city.name,
                markets: {
                    let mut m: HashMap<&str, MarketInfo> = HashMap::new();
                    m.insert("Grain", MarketInfo {
                        supply: market_info.supply,
                        consumption: market_info.consumption,
                        production: market_info.consumption,
                        price: market_info.current_price().0,
                    });
                    m
                },
                links: links.0.iter()
                    .map(|to| to.city.name.as_str())
                    .collect(),
            }
        }).collect(),
        edges: state.nodes.iter().flat_map(|(city, links, _market_info, _pos)| {
            links.0.iter()
                .map(|to| MEdge {
                    nodes: vec![&city.name, &to.city.name]
                })
                .collect::<Vec<_>>()
        }).collect(),
        agents: state.agents.iter().map(|(agent, pos, money, cargo)| {
            MAgent{
                id: &agent.name,
                cargo: &cargo.good.name,
                location: &pos.city().expect("only node agent positions implemented").city.name,
                money: money.0,
            }
        }).collect(),
    }
}

fn state_to_rgraph(state: &State) -> RGraph {
    let m: HashMap<NodeId, RNode> = HashMap::from_iter(
        state.nodes.iter()
            .map(|(city, _links, _market_info, pos)| {
                (city.name.as_str(),
                 RNode {
                     x: pos.0.x as i32,
                     y: pos.0.y as i32,
                     id: &city.name,
                     radius: 1.0,
                 })
            }));
    RGraph {
        nodes: m.values().cloned().collect(),
        edges: state.nodes.iter()
            .flat_map(|(city, links, _market_info, _pos)| {
                links.0.iter()
                    .map(|to| REdge {
                        nodes: (m[city.name.as_str()], m[(to.clone()).city.name.as_str()])
                    }).collect::<Vec<_>>()
            }).collect(),
    }
}

#[derive(Serialize, Debug)]
pub struct Model<'a> {
    tick: u64,
    nodes: Vec<MNode<'a>>,
    edges: Vec<MEdge<'a>>,
    agents: Vec<MAgent<'a>>,
}

#[derive(Serialize, Debug)]
pub struct MarketInfo {
    pub supply: f64,
    pub consumption: f64,
    pub production: f64,
    pub price: f64,
}

pub type NodeId<'a> = &'a str;
pub type AgentId<'a> = &'a str;
pub type Good<'a> = &'a str;

#[derive(Serialize, Debug)]
pub struct MNode<'a> {
    pub id: NodeId<'a>,
    pub markets: HashMap<Good<'a>, MarketInfo>,
    pub links: Vec<NodeId<'a>>,
}

#[derive(Serialize, Debug)]
pub struct MAgent<'a> {
    id: AgentId<'a>,
    cargo: Good<'a>,
    location: NodeId<'a>,
    money: f64,
}

#[derive(Serialize, Debug)]
pub struct MEdge<'a> {
    pub nodes: Vec<NodeId<'a>>
}

#[derive(Serialize, Debug)]
pub struct RGraph<'a> {
    pub nodes: Vec<RNode<'a>>,
    pub edges: Vec<REdge<'a>>,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct RNode<'a> {
    x: i32,
    y: i32,
    id: NodeId<'a>,
    radius: f32,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct REdge<'a> {
    pub nodes: (RNode<'a>, RNode<'a>)
}

