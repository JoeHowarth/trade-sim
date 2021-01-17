use crate::prelude::*;
use crate::State;
use warp::Filter;

pub async fn server(state: Arc<Mutex<State>>) {
    let cors = warp::cors()
        .allow_any_origin();
    // Match any request and return hello world!
    let routes = warp::any()
        .map(move || {
            let state = state.lock().unwrap();
            info!("State is: {:?}", state);
            let model = state_to_model(&state);
            warp::reply::json(&model)
        })
        .with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn state_to_model(state: &State) -> Model {
    Model{
        nodes: state.0.iter().map(|(city, links, market_info)| {
           MNode{
               id: city.name.clone(),
               markets: {
                   let mut m : HashMap<String, MarketInfo> = HashMap::new();
                   m.insert("Grain".to_string(), MarketInfo{
                       supply: market_info.supply,
                       consumption: market_info.consumption,
                       production: market_info.consumption,
                       price: market_info.current_price().0,
                   });
                   m
               },
           }
        }).collect(),
        edges: vec![],
        agents: vec![]
    }
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
