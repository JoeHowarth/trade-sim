use types::{
    market::{
        exchanger::MarketInfo,
        Money,
        exchanger::Order,
    },
    prelude::*,
    City,
    CityHandle,
};
use types::agent::{Agent, Cargo};
use types::market::exchanger::Exchanger;

pub fn clear_orders(
    mut markets: Query<&mut MarketInfo, With<City>>,
    mut agents: Query<(&mut Money, &mut Cargo), With<Agent>>,
    mut orders: EventReader<Order>,
    mut failed_orders: EventWriter<Order>,
) {
    orders.iter().for_each(|order: &Order| {
        let mut market: Mut<MarketInfo> = markets.get_mut(order.market.entity).expect("market entity not in markets query");
        let (mut wallet, mut cargo): (Mut<Money>, Mut<Cargo>) = agents.get_mut(order.agent.entity).expect("agent entity not in agents query");
        match market.buy(&mut wallet, order.amt) {
            Some(_) => {
                cargo.good = order.good;
                cargo.amt = order.amt.max(0) as u32;
            }
            None => failed_orders.send(order.clone())
        }
    })
}
