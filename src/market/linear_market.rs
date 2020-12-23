use crate::prelude::*;
use crate::market::{LinearMarket, Market, MarketInfo, Pricer, Money, Exchanger};
use crate::types::GoodHandle;

impl Market for LinearMarket {
    type MarketInfo = MarketInfo;

    fn price(&self, good: &GoodHandle) -> f64 {
        let info = self.info(good);
        info.pricer.price(info.supply)
    }

    fn cost(&self, good: &GoodHandle, amt: i32) -> f64 {
        self.info(good).cost(amt)
    }

    fn goods(&self) -> hash_map::Keys<GoodHandle, MarketInfo> {
        self.table.keys()
    }

    fn info(&self, good: &GoodHandle) -> &Self::MarketInfo {
        self.table.get(&good)
            .expect(&*format!("Good: {} not found in market", **good))
    }

    fn buy(&mut self, wallet: &mut Money, good: &GoodHandle, amt: i32) -> Option<Money> {
        self.info(good).buy(wallet, amt)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::market::{MarketInfo, LinearMarket, Market, LinearPricer, Pricer};
    use crate::types::{GoodHandle, Good};

    #[test]
    fn linear_market_cost() {
        let pricer = LinearPricer::new(35., 100., -1.);
        let market_info = MarketInfo {
            demand: 30.,
            supply: 35.,
            production: 29.,
            pricer: pricer.clone(),
        };
        let market_info_after = MarketInfo {
            supply: 30.,
            ..market_info.clone()
        };
        let (before, after) = (GoodHandle::from(Good::from("Before")), GoodHandle::from(Good::from("After")));
        let lm: LinearMarket = [(before.clone(), market_info.clone()), (after.clone(), market_info_after.clone())]
            .iter().cloned().collect::<HashMap<GoodHandle, MarketInfo>>().into();

        assert!(lm.cost(&before, 5) > 5. * market_info.current_price(),
                "cost of buying 5 should be more than 5*current_price to avoid buy/sell arbitrage");
        assert_eq!(lm.cost(&before, 2),
                   pricer.price(35.) + pricer.price(34.));
        assert_eq!(lm.cost(&before, 5),
                   std::iter::repeat(35.).enumerate()
                       .map(|(i, s)| pricer.price(s - i as f64))
                       .take(5).sum::<f64>());
        assert_eq!(lm.cost(&before, -2),
                   -1. * (pricer.price(36.) + pricer.price(37.)));

        assert_eq!(lm.cost(&after, -2), -1. * (pricer.price(31.) + pricer.price(32.)));
        assert_eq!(lm.cost(&before, 5), -1. * lm.cost(&after, -5));
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
}