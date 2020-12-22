use crate::{
    prelude::*,
    types::LinearMarket,
};
use crate::types::{Market, GoodHandle, MarketInfo, LinearPricer, Pricer, Good};

impl Market for LinearMarket {
    type MarketInfo = MarketInfo;

    fn cost(&self, good: &GoodHandle, amt: i32) -> f64 {
        if amt == 0 {
            return 0.
        }
        let info = self.info(good);
        let sign = (amt.abs() / amt) as f64;
        assert_eq!(sign.abs(), 1.);
        let amt = amt as f64;
        // if amt is positive, then new supply will be less than current supply
        let future_price = info.pricer.price(info.supply - amt + sign);
        let avg_price = (future_price + info.current_price()) / 2.;
        amt.abs() * avg_price
    }

    fn price(&self, good: &GoodHandle) -> f64 {
        let info = self.info(good);
        info.pricer.price(info.supply)
    }

    fn goods(&self) -> hash_map::Keys<GoodHandle, MarketInfo> {
        self.table.keys()
    }

    fn info(&self, good: &GoodHandle) -> &Self::MarketInfo {
        self.table.get(&good)
            .expect(&*format!("Good: {} not found in market", **good))
    }

    fn buy(&mut self, _good: &GoodHandle, _amt: i32) {
        unimplemented!()
        // let mut market_info = self.table.get_mut(&good)
        //     .expect(&*format!("Good: {} not found in market", *good));
    }
}

impl MarketInfo {
    pub fn current_price(&self) -> f64 {
        self.pricer.price(self.supply)
    }
}

impl Pricer for LinearPricer {
    fn price(&self, amt: f64) -> f64 {
        self.price_per_supply * (amt as f64 - self.base_supply) + self.base_price
    }
}

impl LinearPricer {
    fn new(base_supply: f64, base_price: f64, price_per_supply: f64) -> Self {
        if price_per_supply < 0. {
            warn!("Expected price per supply to be negative, actually: {:?}", price_per_supply);
        }
        LinearPricer { base_price, base_supply, price_per_supply }
    }
}

impl From<Good> for GoodHandle {
    fn from(g: Good) -> Self { GoodHandle { read: Arc::new(g) } }
}

impl<T: Into<String>> From<T> for Good {
    fn from(x: T) -> Self { Good { name: x.into() } }
}

mod tests {
    use crate::prelude::*;
    use crate::types::{LinearMarket, MarketInfo, GoodHandle, LinearPricer, Pricer, Market, Good};

    #[test]
    fn linear_market_cost() {
        let market_info = MarketInfo {
            demand: 30.,
            supply: 35.,
            production: 29.,
            pricer: LinearPricer::new(35., 100., -1.).clone(),
        };
        let gh = GoodHandle::from(Good::from("Grain".to_string()));
        let lm: LinearMarket = [gh.clone()]
            .iter().cloned().map(|gh| (gh, market_info.clone()))
            .collect::<HashMap<GoodHandle, MarketInfo>>().into();
        assert!(lm.cost(&gh, 5) > 5. * market_info.current_price(),
                "cost of buying 5 should be more than 5*current_price to avoid buy/sell arbitrage");
        assert_eq!(lm.cost(&gh, 2),
                   market_info.pricer.price(35.) + market_info.pricer.price(34.));
        assert_eq!(lm.cost(&gh, -2),
                   market_info.pricer.price(35.) + market_info.pricer.price(36.));
    }

    fn goods() -> impl Iterator<Item=Good> {
        ["Wood", "Iron", "Food"].iter()
            .map::<&str, _>(AsRef::as_ref)
            .map(Good::from)
    }

    fn good_handles() -> impl Iterator<Item=GoodHandle> {
        goods()
            .map(GoodHandle::from)
    }

    #[test]
    fn linear_market_basics() {
        let base_price = 100.;
        let good = good_handles().next().unwrap();
        let pricer = LinearPricer::new(35., base_price, -1.);
        let base_supply = 35.;
        let market_info = MarketInfo {
            demand: 30.,
            supply: base_supply,
            production: 29.,
            pricer: pricer.clone(),
        };
        let lm: LinearMarket = good_handles()
            .map(|gh| { (gh, market_info.clone()) })
            .collect::<HashMap<GoodHandle, MarketInfo>>().into();

        // .price(good)
        assert_eq!(lm.price(&good), pricer.price(base_supply));
        // .goods()
        assert_eq!(lm.goods().cloned().collect::<HashSet<GoodHandle>>(),
                   good_handles().collect::<HashSet<GoodHandle>>());
        // .info(good)
        assert_eq!(lm.info(&good), &market_info)
    }

    #[test]
    fn linear_price_pricer() {
        let pricer = LinearPricer::new(50., 10., -2.);
        assert_eq!(pricer.price(51.), 8.);
    }
}