use types::{
    agent::{AgentHandle},
    market::{
        exchanger::MarketInfo,
        Money,
        exchanger::Order,
    },
    prelude::*,
    City,
    CityHandle,
};
use types::agent::Agent;
use types::market::exchanger::Exchanger;

pub fn clear_orders(
    mut markets: Query<&mut MarketInfo, With<City>>,
    mut agents: Query<&mut Money, With<Agent>>,
    mut orders: EventReader<Order>,
    mut failed_orders: EventWriter<Order>,
) {
    orders.iter().for_each(|order: &Order| {
        let mut market: Mut<MarketInfo> = markets.get_mut(order.market.entity).expect("market entity not in markets query");
        let mut agent: Mut<Money> = agents.get_mut(order.agent.entity).expect("agent entity not in agents query");
        match market.buy(&mut agent, order.amt) {
            Some(_) => {}
            None => failed_orders.send(order.clone())
        }
    })
}
