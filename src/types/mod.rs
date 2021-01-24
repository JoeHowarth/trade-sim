pub mod basic_impls;

pub use basic_impls::*;
// types for trade-sim
use crate::prelude::*;
use std::fmt::Formatter;

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
    pub info: City,
}

#[derive(Deref, Debug, Clone)]
pub struct LinkedCities(pub Vec<CityHandle>);

#[derive(Deref, Eq, Clone, Debug, PartialEq, Hash)]
pub struct Good {
    pub name: String,
}

pub struct Agent;

///////////// Market Types ///////////

// see crate::market

///////////// GeoTypes /////////////

#[derive(Deref, Debug, Clone, Copy)]
pub struct Position(pub Vec2);

