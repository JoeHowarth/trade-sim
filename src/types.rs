// types for trade-sim
use crate::prelude::*;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct City {
    pub name: String,
}

#[derive(Deref, Eq, PartialEq, Hash)]
pub struct CityHandle {
    #[deref]
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

#[derive(Deref)]
pub struct LinkedCities(pub Vec<CityHandle>);

#[derive(Deref, Eq, Clone, PartialEq, Hash)]
pub struct Good {
    name: String,
}

#[derive(Deref, Eq, Clone, PartialEq, Hash)]
pub struct GoodHandle {
    read: Arc<Good>,
}

pub struct Agent;

///////////// Market Types ///////////

pub trait Market {
    type MarketInfo;
    fn price(good: GoodHandle) -> f32;
    fn buy(&mut self, good: GoodHandle, amt: i32);
    fn goods(&self) -> std::slice::Iter<'_, GoodHandle>;
    fn info(&self, good: GoodHandle) -> &Self::MarketInfo;
    fn sell(&mut self, good: GoodHandle, amt: i32) {
        return self.buy(good, -amt);
    }
}

#[derive(Debug)]
pub struct MarketInfo;

pub struct LinearMarket {
    pub table: std::collections::HashMap<GoodHandle, MarketInfo>
}

////////////// Display Types ///////////////

#[derive(Deref, Debug, From)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: impl Into<Vec2>) -> Self {
        return Self(x.into());
    }
}
