mod impls;

pub use impls::*;
use crate::{
    prelude::*,
    *,
};
use rand::prelude::SmallRng;
use crate::market::exchanger::{MarketInfo, Exchanger};
use crate::market::{Money, LinearMarket};

/*
Agent Components:
- Agent
- GraphPosition
- Cargo
- Money
 */

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct AgentHandle {
    pub agent: Agent,
    pub entity: Entity,
}

#[derive(Debug, From, Clone, Eq, PartialEq, Hash)]
pub struct Agent {
    pub name: String,
}

#[derive(Debug, From, Clone, Eq, PartialEq, Hash)]
pub struct Cargo {
    pub good: Good,
    pub amt: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GraphPosition {
    Node(CityHandle),
    Edge(CityHandle, CityHandle),
}

/*
Functions
 */
