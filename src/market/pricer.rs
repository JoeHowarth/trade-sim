use crate::prelude::*;
use crate::market::Money;

pub trait Pricer {
    fn price(&self, amt: f64) -> Money;
}

#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
pub struct LinearPricer {
    pub base_supply: f64,
    pub base_price: f64,
    pub price_per_supply: f64,
}

impl Pricer for LinearPricer {
    fn price(&self, amt: f64) -> Money {
        (self.price_per_supply * (amt as f64 - self.base_supply) + self.base_price).into()
    }
}

impl LinearPricer {
    pub fn new(base_supply: f64, base_price: f64, price_per_supply: f64) -> Self {
        if price_per_supply < 0. {
            warn!("Expected price per supply to be negative, actually: {:?}", price_per_supply);
        }
        LinearPricer { base_price, base_supply, price_per_supply }
    }
}
