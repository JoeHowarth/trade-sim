use types::{market::{
    exchanger::MarketInfo,
    Money,
    exchanger::Order,
}, prelude::*, City, CityHandle, EntityMapMut};
use types::agent::{Agent, Cargo};
use types::market::exchanger::Exchanger;

pub fn clear_orders(
    mut markets: Query<(Entity, &mut MarketInfo), With<City>>,
    mut agents: Query<(Entity, &mut Money, &mut Cargo), With<Agent>>,
    mut orders: EventReader<Order>,
    mut failed_orders: EventWriter<Order>,
    m: Mut<Money>,
) {

    let mut agents = HashMap::<Entity,(M_>::from_iter(agents.iter_mut().map(|(e,m,c)| (e, (m,c))));
    let mut markets: EntityMapMut<'_, MarketInfo> = HashMap::from_iter(markets.iter_mut());

    orders.iter().for_each(|order: &Order| {
        let mut market = markets.get_mut(&order.market.entity).expect("market entity not in markets query");
        let (mut wallet, mut cargo) = agents.get_mut(&order.agent.entity).expect("agent entity not in agents query");
        match market.buy(&mut wallet, order.amt) {
            Some(_) => {
                cargo.good = order.good;
                cargo.amt = order.amt.max(0) as u32;
            }
            None => failed_orders.send(order.clone())
        }
    })
}
