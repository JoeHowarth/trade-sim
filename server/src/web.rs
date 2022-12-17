use types::{prelude::*, State};
use crate::modelserver;

pub(crate) fn state_to_model(state: &State) -> modelserver::Model {
    modelserver::Model {
        tick: state.tick.0,
        nodes: state
            .nodes
            .iter()
            .map(|(city, links, market_info, _pos)| {
                (
                    city.name.to_string(),
                    modelserver::Node {
                        id: city.name.to_string(),
                        markets: {
                            let mut m: HashMap<String, modelserver::MarketInfo> = HashMap::new();
                            m.insert(
                                "Grain".to_string(),
                                modelserver::MarketInfo {
                                    supply: market_info.supply,
                                    consumption: market_info.consumption,
                                    production: market_info.production,
                                    price: market_info.current_price().0,
                                },
                            );
                            m
                        },
                        links: links.0.iter().map(|to| to.city.name.to_string()).collect(),
                    },
                )
            })
            .collect(),
        edges: state
            .nodes
            .iter()
            .flat_map(|(city, links, _market_info, _pos)| {
                links
                    .0
                    .iter()
                    .map(|to| modelserver::Edge {
                        from: city.name.to_string(),
                        to: to.city.name.to_string(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        agents: state
            .agents
            .iter()
            .map(|(agent, pos, money, cargo)| {
                (
                    agent.name.to_string(),
                    modelserver::Agent {
                        id: agent.name.to_string(),
                        cargo: cargo.good.name.to_string(),
                        location: pos
                            .city()
                            .expect("only node agent positions implemented")
                            .city
                            .name.to_string(),
                        money: money.0,
                    },
                )
            })
            .collect(),
    }
}

pub(crate) fn state_to_rgraph(state: &State) -> modelserver::RGraph {
    modelserver::RGraph {
        nodes: HashMap::from_iter(state.nodes.iter().map(
            |(city, _links, _market_info, pos)| {
                (
                    city.name.to_string(),
                    modelserver::RNode {
                        x: pos.0.x as i32,
                        y: pos.0.y as i32,
                        id: city.name.to_string(),
                        radius: 1.0,
                    },
                )
            },
        )),
        edges: state
            .nodes
            .iter()
            .flat_map(|(city, links, _market_info, _pos)| {
                links
                    .0
                    .iter()
                    .map(|to| modelserver::Edge {
                        from: city.name.to_string(),
                        to: to.city.name.to_string(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    }
}

// #[derive(Serialize, Deserialize)]
// pub(crate) struct SaveFormat {
//     pub(crate) models: BTreeMap<u64, Model>,
//     pub(crate) visual: RGraph,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Model {
//     pub tick: u64,
//     nodes: HashMap<NodeId, MNode>,
//     edges: Vec<MEdge>,
//     agents: HashMap<AgentId, MAgent>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct MarketInfo {
//     pub supply: f64,
//     pub consumption: f64,
//     pub production: f64,
//     pub price: f64,
// }

// pub type NodeId = Ustr;
// pub type AgentId = Ustr;
// pub type Good = Ustr;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct MNode {
//     pub id: NodeId,
//     pub markets: HashMap<Good, MarketInfo>,
//     pub links: Vec<NodeId>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct MAgent {
//     id: AgentId,
//     cargo: Good,
//     location: NodeId,
//     money: f64,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct MEdge {
//     pub nodes: Vec<NodeId>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct RGraph {
//     pub nodes: BTreeMap<NodeId, RNode>,
//     pub edges: Vec<REdge>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
// pub struct RNode {
//     x: i32,
//     y: i32,
//     id: NodeId,
//     radius: f32,
// }

// #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
// pub struct REdge {
//     pub nodes: (NodeId, NodeId),
// }
