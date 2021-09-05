use types::prelude::*;
use types::State;

pub(crate) fn state_to_model(state: &State) -> Model {
    Model {
        tick: state.tick.0,
        nodes: state.nodes.iter().map(|(city, links, market_info, _pos)| {
            (city.name, MNode {
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
            })
        }).collect(),
        edges: state.nodes.iter().flat_map(|(city, links, _market_info, _pos)| {
            links.0.iter()
                .map(|to| MEdge {
                    nodes: vec![city.name, to.city.name]
                })
                .collect::<Vec<_>>()
        }).collect(),
        agents: state.agents.iter().map(|(agent, pos, money, cargo)| {
            (agent.name, MAgent {
                id: agent.name,
                cargo: cargo.good.name,
                location: pos.city().expect("only node agent positions implemented").city.name,
                money: money.0,
            })
        }).collect(),
    }
}

pub(crate) fn state_to_rgraph(state: &State) -> RGraph {
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
    pub tick: u64,
    nodes: HashMap<NodeId, MNode>,
    edges: Vec<MEdge>,
    agents: HashMap<AgentId, MAgent>,
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

