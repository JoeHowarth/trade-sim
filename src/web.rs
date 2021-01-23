use crate::prelude::*;
use crate::State;
use warp::{Filter, Rejection, Reply};
use std::convert::Infallible;
use std::future::Future;
use std::result::Result;
use tokio::sync::watch;
use std::borrow::Borrow;

pub async fn server(state: watch::Receiver<State>) {
    let cors = warp::cors()
        .allow_any_origin();
    // Match any request and return hello world!
    // let base = warp::any()
    //     .and(warp::any().map(move || state.clone()));
    let other_state = state.clone();
    let state_route = warp::path("state")
        .and(warp::any().map(move || state.clone()))
        .and_then(handler);
    let rgraph_route = warp::path("rgraph")
        .and(warp::any().map(move || other_state.clone()))
        .and_then(rgraph_handler);

    let full = state_route.or(rgraph_route).with(cors);

    warp::serve(full).run(([127, 0, 0, 1], 3030)).await;
}

async fn handler(state: watch::Receiver<State>) -> Result<impl Reply, Rejection> {
    let state = state.borrow();
    info!("State is: {:?}", state);
    let model = state_to_model(&state);
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
        nodes: state.0.iter().map(|(city, links, market_info, _pos)| {
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
                    .map(|to| to.info.name.as_str())
                    .collect(),
            }
        }).collect(),
        edges: state.0.iter().flat_map(|(city, links, _market_info, _pos)| {
            links.0.iter()
                .map(|to| MEdge {
                    nodes: vec![&city.name, &to.info.name]
                })
                .collect::<Vec<_>>()
        }).collect(),
        agents: vec![],
    }
}

fn state_to_rgraph(state: &State) -> RGraph {
    let m: HashMap<NodeId, RNode> = HashMap::from_iter(
        state.0.iter()
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
        edges: state.0.iter()
            .flat_map(|(city, links, _market_info, _pos)| {
                links.0.iter()
                    .map(|to| REdge {
                        nodes: (m[city.name.as_str()], m[(to.clone()).info.name.as_str()])
                    }).collect::<Vec<_>>()
            }).collect(),
    }
}

#[derive(Serialize, Debug)]
pub struct Model<'a> {
    nodes: Vec<MNode<'a>>,
    edges: Vec<MEdge<'a>>,
    agents: Vec<()>,
}

#[derive(Serialize, Debug)]
pub struct MarketInfo {
    pub supply: f64,
    pub consumption: f64,
    pub production: f64,
    pub price: f64,
}

pub type NodeId<'a> = &'a str;
pub type Good<'a> = &'a str;

#[derive(Serialize, Debug)]
pub struct MNode<'a> {
    pub id: NodeId<'a>,
    pub markets: HashMap<Good<'a>, MarketInfo>,
    pub links: Vec<NodeId<'a>>,
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

