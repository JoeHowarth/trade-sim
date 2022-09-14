mod impls;

use crate::{
    market::{
        exchanger::{Exchanger, MarketInfo},
        LinearMarket, Money,
    },
    prelude::*,
    *,
};
pub use impls::*;
use rand::prelude::SmallRng;

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

#[derive(
    Component, Debug, From, Clone, Eq, PartialEq, Hash, Copy,
)]
pub struct Agent {
    pub name: Ustr,
}

#[derive(
    Component, Debug, From, Clone, Copy, Eq, PartialEq, Hash,
)]
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
