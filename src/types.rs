// types for trade-sim
use crate::prelude::*;
use std::fmt::Formatter;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct City {
    pub name: String,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct CityHandle {
    pub entity: Entity,
    pub info: City,
}

impl Clone for CityHandle {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            info: self.info.clone(),
        }
    }
}

#[derive(Deref, Debug)]
pub struct LinkedCities(pub Vec<CityHandle>);

#[derive(Deref, Eq, Clone, Debug, PartialEq, Hash)]
pub struct Good {
    pub name: String,
}

#[derive(Deref, Eq, Clone, Debug, PartialEq, Hash)]
pub struct GoodHandle {
    pub read: Arc<Good>,
}

pub struct Agent;

///////////// Market Types ///////////

pub trait Market {
    type MarketInfo;
    fn price(&self, good: &GoodHandle) -> f64;
    fn cost(&self, good: &GoodHandle, amt: i32) -> f64;
    fn goods(&self) -> hash_map::Keys<GoodHandle, MarketInfo>;
    fn info(&self, good: &GoodHandle) -> &Self::MarketInfo;
    fn buy(&mut self, good: &GoodHandle, amt: i32);
    fn sell(&mut self, good: &GoodHandle, amt: i32) {
        return self.buy(good, -amt);
    }
}

pub trait Pricer {
    fn price(&self, amt: f64) -> f64;
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct MarketInfo{
    pub demand: f64,
    pub supply: f64,
    pub production: f64,
    pub pricer: LinearPricer
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct LinearPricer {
    pub base_supply: f64,
    pub base_price: f64,
    pub price_per_supply: f64,
}

#[derive(From, Debug)]
pub struct LinearMarket {
    pub table: HashMap<GoodHandle, MarketInfo>
}

////////////// Display Types ///////////////

#[derive(Deref, Debug, From)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: impl Into<Vec2>) -> Self {
        return Self(x.into());
    }
}

impl std::fmt::Display for Good {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}