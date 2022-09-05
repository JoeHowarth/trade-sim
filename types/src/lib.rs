#![allow(unused_imports, dead_code)]
pub mod agent;
pub mod basic_impls;
pub mod market;
pub mod prelude;
pub mod query_like;
pub mod utility;

use crate::{
    agent::{Agent, AgentHandle, Cargo, GraphPosition},
    market::{exchanger::MarketInfo, Money},
    prelude::*,
};
pub use basic_impls::*;
use bevy::prelude::{Entity, Vec2};
pub use derive_more::{
    Add, AddAssign, Deref, Display, Div, From, Into, Mul, MulAssign,
    Sub, SubAssign, Sum,
};
pub use serde::{Deserialize, Serialize};
use std::{cell, collections::HashSet, fmt::Formatter, sync::atomic};

#[derive(Debug, From, Clone, PartialEq)]
pub enum Action {
    Movement(Movement),
    Order(Order),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    pub good: crate::Good,
    pub market: CityHandle,
    pub agent: AgentHandle,
    /// positive amt means buy order, negative means sell order
    pub amt: i32,
}

#[derive(Debug, From, Clone, Copy, PartialEq)]
pub struct Movement {
    pub from: GraphPosition,
    pub to: GraphPosition,
    pub entity: Entity,
}

#[derive(Debug)]
pub struct State {
    pub tick: Tick,
    pub nodes: Vec<(City, LinkedCities, MarketInfo, GridPosition)>,
    pub agents: Vec<(Agent, GraphPosition, Money, Cargo)>,
}

///////// Infrastructure Types /////////

#[derive(
    Component, Debug, From, Clone, Copy, Eq, PartialEq, Hash,
)]
pub struct Tick(pub u64);

//////// General Simulation Types ////////

#[derive(
    Component, Debug, From, Clone, Copy, Eq, PartialEq, Hash,
)]
pub struct City {
    pub name: Ustr,
}

#[derive(Component, Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct CityHandle {
    pub entity: Entity,
    pub city: City,
}

#[derive(Component, Deref, Debug, Clone)]
pub struct LinkedCities(pub Vec<CityHandle>);

#[derive(
    Component, Deserialize, Eq, Clone, Copy, Debug, PartialEq, Hash,
)]
#[serde(transparent)]
pub struct Good {
    pub name: Ustr,
}

#[derive(Component, Deref, Debug, Clone)]
// Resource representing all known goods
pub struct Goods(pub HashSet<Good>);

///////////// Market Types ///////////

// see crate::market

///////////// GeoTypes /////////////

#[derive(Component, Deref, PartialEq, Debug, Clone, Copy)]
pub struct GridPosition(pub Vec2);
