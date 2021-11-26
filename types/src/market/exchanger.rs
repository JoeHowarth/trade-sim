use std::ops::DerefMut;
use crate::prelude::*;
use crate::market::{money::Money, pricer::{LinearPricer, Pricer}};
use crate::CityHandle;
use crate::agent::AgentHandle;

pub trait Exchanger {
    fn cost(&self, amt: i32) -> Money;
    fn dry_run_by(&self, wallet: &mut Money, amt: i32) -> Option<Money>;
    fn buy(&mut self, wallet: &mut Money, amt: i32) -> Option<Money>;
    fn sell(&mut self, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.buy(wallet, -amt)
    }
}

#[derive(Debug, Clone)]
pub struct DryRunExchanger<'a, T: Exchanger> {
    pub inner: &'a T,
}

impl<'a, T: Exchanger> Exchanger for DryRunExchanger<'a, T> {
    fn cost(&self, amt: i32) -> Money {
        self.inner.cost(amt)
    }
    fn dry_run_by(&self, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.inner.dry_run_by(wallet, amt)
    }
    fn buy(&mut self, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.inner.dry_run_by(wallet, amt)
    }
}

#[derive(Deserialize, Debug, PartialOrd, PartialEq, Clone)]
pub struct MarketInfo {
    pub consumption: f64,
    pub supply: f64,
    pub production: f64,
    pub pricer: LinearPricer,
}

impl Exchanger for Mut<'_, MarketInfo> {
    fn cost(&self, amt: i32) -> Money {
        self.deref().cost(amt)
    }

    fn dry_run_by(&self, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.deref().dry_run_by(wallet, amt)
    }

    fn buy(&mut self, wallet: &mut Money, amt: i32) -> Option<Money> {
        self.deref_mut().buy(wallet, amt)
    }
}

impl Exchanger for MarketInfo {
    fn cost(&self, amt: i32) -> Money {
        // Invariant: cost of buying followed by selling the same number must sum to 0.
        if amt == 0 {
            return 0.0.into();
        }
        let p = |amt| self.pricer.price(amt);
        let avg_price = if amt > 0 {
            (p(self.supply) + p(self.supply - amt as f64 + 1.)) / 2.
        } else {
            (p(self.supply + 1.) + p(self.supply - amt as f64)) / 2.
        };
        (avg_price * amt as f64).into()
    }

    fn dry_run_by(&self, wallet: &mut Money, amt: i32) -> Option<Money> {
        if amt == 0 { return Some(0.0.into()); }
        let cost = self.cost(amt);
        if cost > *wallet { return None; }
        *wallet -= cost;
        Some(cost.into())
    }

    /// `buy` takes a mutable `wallet` and an amount, `amt`, to buy and performs the transaction if possible
    /// if cost is greater than contents of wallet, return None
    /// the cost of the transaction is removed from `wallet` and the cost is returned
    /// the supply of goods is decreased by `amt`
    fn buy(&mut self, wallet: &mut Money, amt: i32) -> Option<Money> {
        let cost = self.dry_run_by(wallet, amt)?;
        self.supply -= amt as f64;
        Some(cost)
    }
}

impl MarketInfo {
    pub fn current_price(&self) -> Money {
        self.pricer.price(self.supply)
    }
    pub fn produce_and_consume(&mut self) {
        self.supply = self.supply + self.production - self.consumption
    }
}

mod tests {
    use crate::market::pricer::{LinearPricer, Pricer};
    use crate::market::exchanger::{MarketInfo, Exchanger};
    use crate::market::Money;

    #[test]
    fn linear_price_pricer() {
        let pricer = LinearPricer::new(50., 10., -2.);
        assert_eq!(pricer.price(51.), 8.0.into());
    }

    #[test]
    fn buy_sell() {
        let pricer = LinearPricer::new(35., 100., -1.);
        let mut market_info = MarketInfo {
            consumption: 30.,
            supply: 35.,
            production: 29.,
            pricer: pricer.clone(),
        };

        let starting_balance = Money(10_000.);
        for &amt in [1., 10.].iter() {
            let mut wallet: Money = starting_balance;
            let initial_cost = market_info.cost(amt as i32);
            let initial_money = Some(initial_cost.into());

            assert_eq!(market_info.buy(&mut wallet, amt as i32), initial_money);
            assert_eq!(wallet, starting_balance - initial_money.unwrap());

            assert_eq!(market_info.sell(&mut wallet, amt as i32), initial_money.map(Money::rneg));
            assert_eq!(wallet, starting_balance);
        }
    }
}