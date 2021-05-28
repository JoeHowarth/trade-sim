pub mod exchanger;
pub mod linear_market;
pub mod pricer;
pub mod money;

use crate::{
    prelude::*,
    *,
    market::{
        exchanger::{Exchanger, MarketInfo}
    },
};
use std::fmt::Debug;

pub use crate::market::money::Money;
use crate::Good;

pub trait Market {
    type MarketInfo: Exchanger;
    // shared
    fn price(&self, good: &Good) -> Money;
    fn cost(&self, good: &Good, amt: i32) -> Money;
    fn goods(&self) -> hash_map::Keys<Good, Self::MarketInfo>;
    fn info(&self, good: &Good) -> &Self::MarketInfo;
    // exclusive
    fn info_mut(&mut self, good: &Good) -> &mut Self::MarketInfo;
    fn buy(&mut self, good: &Good, wallet: &mut Money, amt: i32) -> Option<Money>;
    fn sell(&mut self, good: &Good, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.buy(good, wallet, -amt)
    }
}

#[derive(From, Debug)]
pub struct LinearMarket {
    pub table: HashMap<Good, MarketInfo>
}


