pub mod exchanger;
mod linear_market;
mod pricer;

use crate::{
    prelude::*,
    types::*,
    market::exchanger::{Exchanger, MarketInfo},
};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Mul};

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

#[derive(Add, Sum, Sub, SubAssign, Div, AddAssign, MulAssign, From, Into, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Money(pub f64);

impl Money {
    pub fn neg(&self) -> Self {
        Money(-1. * self.0)
    }
    pub fn rneg<T: AsRef<Self>>(t: T) -> Self {
        Money(-1. * (t.as_ref()).0)
    }
}

impl<__RhsT> ::core::ops::Mul<__RhsT> for Money
    where
        f64: ::core::ops::Mul<__RhsT, Output = f64>,
{
    type Output = Money;
    #[inline]
    fn mul(self, rhs: __RhsT) -> Money {
        Money(<f64 as ::core::ops::Mul<__RhsT>>::mul(self.0, rhs))
    }
}

impl Mul<Money> for f64 {
    type Output = Money;

    #[inline]
    fn mul(self, rhs: Money) -> Money {
        Money(rhs.0 * self)
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

