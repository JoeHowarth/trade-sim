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

#[derive(Component, Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct AgentHandle {
    pub agent: Agent,
    pub entity: Entity,
}

#[derive(Component, Debug, From, Clone, Eq, PartialEq, Hash, Copy)]
pub struct Agent {
    pub name: Ustr,
}

#[derive(Component, Debug, From, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Cargo {
    pub good: Good,
    pub amt: u32,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GraphPosition {
    Node(CityHandle),
    Edge(CityHandle, CityHandle),
}

/*
Functions
 */
