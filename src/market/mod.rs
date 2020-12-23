pub mod exchanger;
pub mod linear_market;
pub mod pricer;
pub mod money;

use crate::{
    prelude::*,
    types::*,
    market::{
        exchanger::{Exchanger, MarketInfo}
    },
};
use std::fmt::Debug;

pub use crate::market::money::Money;

pub trait Market {
    type MarketInfo: Exchanger;
    // shared
    fn price(&self, good: &GoodHandle) -> Money;
    fn cost(&self, good: &GoodHandle, amt: i32) -> Money;
    fn goods(&self) -> hash_map::Keys<GoodHandle, Self::MarketInfo>;
    fn info(&self, good: &GoodHandle) -> &Self::MarketInfo;
    // exclusive
    fn info_mut(&mut self, good: &GoodHandle) -> &mut Self::MarketInfo;
    fn buy(&mut self, good: &GoodHandle, wallet: &mut Money, amt: i32) -> Option<Money>;
    fn sell(&mut self, good: &GoodHandle, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.buy(good, wallet, -amt)
    }
}

#[derive(From, Debug)]
pub struct LinearMarket {
    pub table: HashMap<GoodHandle, MarketInfo>
}


