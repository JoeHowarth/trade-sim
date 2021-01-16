use crate::prelude::*;
use warp::Filter;

pub async fn server(state: Arc<Mutex<i32>>) {

    // Match any request and return hello world!
    let routes = warp::any()
        .map(move || {
            let model = Model {
                nodes: vec![],
                edges: vec![],
                agents: vec![]
            };
            format!("Count is: {:?}", state.lock());
            warp::reply::json(&model)
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}


#[derive(Serialize, Debug)]
pub struct RGraph {
    pub nodes: Vec<RNode>,
    pub edges: Vec<REdge>,
}

#[derive(Serialize, Debug)]
pub struct RNode {
    x: i32,
    y: i32,
    id: String,
    radius: f32,
}

#[derive(Serialize, Debug)]
pub struct REdge {
    pub nodes: (RNode, RNode)
}

#[derive(Serialize, Debug)]
pub struct Model {
    nodes: Vec<MNode>,
    edges: Vec<MEdge>,
    agents: Vec<()>,
}

#[derive(Serialize, Debug)]
pub struct MarketInfo {
    pub supply: f64,
    pub consumption: f64,
    pub production: f64,
    pub price: f64,
}

pub type NodeId = String;
pub type Good = String;

#[derive(Serialize, Debug)]
pub struct MNode {
    pub id: NodeId,
    pub markets: HashMap<Good, MarketInfo>,
}

#[derive(Serialize, Debug)]
pub struct MEdge {
    pub nodes: Vec<NodeId>
}
