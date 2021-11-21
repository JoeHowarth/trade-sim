#![allow(unused_imports, dead_code)]
pub mod basic_impls;
pub mod market;
pub mod agent;
pub mod prelude;
pub mod utility;

pub use basic_impls::*;
pub use market::exchanger::Order;
use std::{
    fmt::{Formatter},
    collections::HashSet,
    cell,
    sync::atomic,
};
use bevy::prelude::{Entity, Vec2};
pub use derive_more::{Deref, Add, AddAssign, Sum, Mul, MulAssign, Sub, SubAssign, Div, Display, From, Into};
pub use serde::{Serialize, Deserialize};
use crate::{
    agent::{Agent, GraphPosition, Cargo},
    market::{
        Money,
        exchanger::MarketInfo,
    },
    prelude::*,
};

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

#[derive(Debug, From, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Tick(pub u64);

//////// General Simulation Types ////////

#[derive(Debug, From, Clone, Copy, Eq, PartialEq, Hash)]
pub struct City {
    pub name: Ustr,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct CityHandle {
    pub entity: Entity,
    pub city: City,
}

#[derive(Deref, Debug, Clone)]
pub struct LinkedCities(pub Vec<CityHandle>);

#[derive(Deserialize, Eq, Clone, Copy, Debug, PartialEq, Hash)]
#[serde(transparent)]
pub struct Good {
    pub name: Ustr,
}

#[derive(Deref, Debug, Clone)]
// Resource representing all known goods
pub struct Goods(pub HashSet<Good>);


///////////// Market Types ///////////

// see crate::market

///////////// GeoTypes /////////////

#[derive(Deref, PartialEq, Debug, Clone, Copy)]
pub struct GridPosition(pub Vec2);

