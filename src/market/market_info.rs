use crate::prelude::*;
use crate::types::{GoodHandle};
use crate::market::{LinearPricer, Pricer, MarketInfo, Money, Exchanger};


impl Exchanger for MarketInfo {
    fn cost(&self, amt: i32) -> f64 {
        // Invariant: cost of buying followed by selling the same number must sum to 0.
        if amt == 0 {
            return 0.;
        }
        let p = |amt| self.pricer.price(amt);
        let avg_price = if amt > 0 {
            (p(self.supply) + p(self.supply - amt as f64 + 1.)) / 2.
        } else {
            (p(self.supply + 1.) + p(self.supply - amt as f64)) / 2.
        };
        avg_price * amt as f64
    }

    /// `buy` takes a mutable `wallet` and an amount, `amt`, to buy and performs the transaction if possible
    /// if cost is greater than contents of wallet, return None
    /// the cost of the transaction is removed from `wallet` and the cost is returned
    /// the supply of goods is decreased by `amt`
    fn buy(&mut self, wallet: &mut Money, amt: i32) -> Option<Money> {
        if amt == 0 { return Some((0.).into()); }
        let cost = self.cost(amt);
        if cost > wallet.0 { return None; }
        wallet.0 -= cost;
        self.supply -= amt as f64;
        Some(cost.into())
    }
}

impl MarketInfo {
    pub fn current_price(&self) -> f64 {
        self.pricer.price(self.supply)
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

mod tests {
    use crate::market::market_info::{LinearPricer, MarketInfo};
    use crate::market::{Money, Pricer};

    #[test]
    fn linear_price_pricer() {
        let pricer = LinearPricer::new(50., 10., -2.);
        assert_eq!(pricer.price(51.), 8.);
    }

    #[test]
    fn buy_sell() {
        let pricer = LinearPricer::new(35., 100., -1.);
        let mut market_info = MarketInfo {
            demand: 30.,
            supply: 35.,
            production: 29.,
            pricer: pricer.clone(),
        };

        let starting_balance = Money::from(10.);
        for &amt in [1., 10.].iter() {
            let mut wallet: Money = starting_balance;
            let initial_cost = market_info.cost(amt as i32);
            let initial_money = Some(initial_cost.into());

            assert_eq!(market_info.buy(&mut wallet, amt as i32), initial_money);
            assert_eq!(wallet, starting_balance - initial_money.unwrap());

            assert_eq!(market_info.sell(&mut wallet, amt as i32), initial_money.map(Money::neg));
            assert_eq!(wallet, starting_balance);
        }
    }
}