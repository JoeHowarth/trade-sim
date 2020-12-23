pub mod market_info;
mod linear_market;

pub use crate::types::*;

use crate::{
    prelude::*,
};
use std::cmp::Ordering;
use std::fmt::Debug;

pub trait Market {
    type MarketInfo: Exchanger;
    // shared
    fn price(&self, good: &GoodHandle) -> f64;
    fn cost(&self, good: &GoodHandle, amt: i32) -> f64;
    fn goods(&self) -> hash_map::Keys<GoodHandle, Self::MarketInfo>;
    fn info(&self, good: &GoodHandle) -> &Self::MarketInfo;
    // exclusive
    fn buy(&mut self, good: &GoodHandle, wallet: &mut Money, amt: i32) -> Option<Money>;
    fn sell(&mut self, good: &GoodHandle, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.buy(good, wallet, -amt)
    }
}


#[derive(From, Debug)]
pub struct LinearMarket {
    pub table: HashMap<GoodHandle, MarketInfo>
}

pub trait Pricer {
    fn price(&self, amt: f64) -> f64;
}

impl Pricer for LinearPricer {
    fn price(&self, amt: f64) -> f64 {
        self.price_per_supply * (amt as f64 - self.base_supply) + self.base_price
    }
}

pub trait Exchanger {
    fn cost(&self, amt: i32) -> Money;
    fn buy(&mut self, wallet: &mut Money, amt: i32) -> Option<Money>;
    fn sell(&mut self, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.buy(wallet, -amt)
    }
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

#[derive(Add, Sub, Mul, Div, From, Into, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Money(pub f64);

impl Money {
    pub fn neg<T: AsRef<Self>>(t: T) -> Self {
        Money(-1. * (t.as_ref()).0)
    }
}

impl AsRef<Money> for Money {
    fn as_ref(&self) -> &Money {
        self
    }
}

impl Eq for Money {}

impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect(&*format!("Failed ordering {:?} cmp {:?}", self, other))
    }
}

