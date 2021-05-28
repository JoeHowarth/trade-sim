pub mod basic_impls;

pub use basic_impls::*;
// types for trade-sim
use std::fmt::Formatter;
use std::collections::HashSet;
use bevy::prelude::{Entity, Vec2};
pub use derive_more::{Deref, Add, AddAssign, Sum, Mul, MulAssign, Sub, SubAssign, Div, Display, From, Into};
pub use serde::{Serialize, Deserialize};
use crate::market::exchanger::MarketInfo;
use crate::market::Money;
use crate::agent::{Agent, GraphPosition, Cargo};

#[derive(Debug)]
pub struct State {
    pub tick: Tick,
    pub nodes: Vec<(City, LinkedCities, MarketInfo, GridPosition)>,
    pub agents: Vec<(Agent, GraphPosition, Money, Cargo)>
}

///////// Infrastructure Types /////////

#[derive(Debug, From, Clone, Eq, PartialEq, Hash)]
pub struct Tick(pub u64);

//////// General Simulation Types ////////

#[derive(Debug, From, Clone, Eq, PartialEq, Hash)]
pub struct City {
    pub name: String,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct CityHandle {
    pub entity: Entity,
    pub city: City,
}

#[derive(Deref, Debug, Clone)]
pub struct LinkedCities(pub Vec<CityHandle>);

#[derive(Deserialize, Eq, Clone, Debug, PartialEq, Hash)]
#[serde(transparent)]
pub struct Good {
    pub name: String,
    // #[serde(default = "default_entity")]
    // pub entity: Entity,
}

fn default_entity() -> Entity {
    Entity::new(0)
}


#[derive(Deref, Debug, Clone)]
// Resource representing all known goods
pub struct Goods(pub HashSet<Good>);


///////////// Market Types ///////////

// see crate::market

///////////// GeoTypes /////////////

#[derive(Deref, Debug, Clone, Copy)]
pub struct GridPosition(pub Vec2);

