use types::prelude::*;
use types::agent::{Agent, Cargo, GraphPosition, AgentHandle};
use types::market::{Money};
use types::market::exchanger::{MarketInfo, Exchanger};
use types::{City, CityHandle, ecs_err, LinkedCities};
use std::cmp::Ordering;
use types::prelude::hash_map::RandomState;

pub fn decide(
    agent_q: Query<(Entity, &Agent, &Cargo, &Money, &GraphPosition)>,
    cities_q: Query<(Entity, &City, &MarketInfo, &LinkedCities)>,
    mut orders: EventWriter<Order>,
    mut movement: EventWriter<Movement>,
) -> Result<()> {
    for a in agent_q.iter() {
        decide_single_simple(a, &cities_q, &mut orders, &mut movement)?;
    }
    Ok(())
}


fn decide_single_good_multi_step(
    // (e_agent, agent, cargo, money, pos): (Entity, &Agent, &Cargo, &Money, &GraphPosition),
    // cities_q: &Query<(Entity, &City, &MarketInfo, &LinkedCities)>,
    // orders: &mut EventWriter<Order>,
    // movement: &mut EventWriter<Movement>,
) -> Result<()> {
    Ok(())
}

fn decide_single_simple(
    (e_agent, agent, cargo, _money, pos): (Entity, &Agent, &Cargo, &Money, &GraphPosition),
    cities_q: &Query<(Entity, &City, &MarketInfo, &LinkedCities)>,
    orders: &mut EventWriter<Order>,
    movement: &mut EventWriter<Movement>,
) -> Result<()> {
    let good = Good::from("Grain");
    let agent_handle = AgentHandle { agent: *agent, entity: e_agent };

    // sell cargo if present
    if cargo.amt > 0 {
        orders.send(Order {
            good: cargo.good,
            market: pos.city_res()?,
            agent: agent_handle,
            amt: -(cargo.amt as i32),
        });
    }

    let (_, src_city, src_market, links): (_, &City, &MarketInfo, &LinkedCities) = cities_q.get(pos.city_res()?.entity)?;
    let linked_markets: Vec<(CityHandle, &MarketInfo)> = links.0.iter()
        .map(|ch| (*ch, cities_q.get_component::<MarketInfo>(ch.entity).unwrap()))
        .collect();

    // small chance to move randomly
    let mut rng = SmallRng::from_entropy();
    if rng.gen_bool(0.1) {
        if let Some((dest, _)) = linked_markets.choose(&mut rng) {
            info!("{} randomly moving from {} to {}", agent, src_city, dest);
            movement.send(Movement {
                from: *pos,
                to: GraphPosition::Node(*dest),
                entity: e_agent,
            });
            return Ok(());
        }
    }

    // find neighbor with lowest price
    linked_markets.iter()
        .max_by(|(_, a), (_, b)| {
            a.current_price().cmp(&b.current_price())
        })
        .and_then(|(city_with_highest_price, highest_market)| {
            if highest_market.current_price() > src_market.current_price() {
                // buy good in src_city and move to highest_city
                orders.send(Order {
                    good,
                    market: pos.city()?,
                    agent: agent_handle,
                    amt: 1,
                });
                movement.send(Movement {
                    from: pos.clone(),
                    to: GraphPosition::Node(city_with_highest_price.clone()),
                    entity: e_agent,
                });
                info!("Agent {} buying {} at {} in {} and moving to {}, expecting profit of {}",
                    agent, good, src_market.current_price(), src_city, city_with_highest_price, highest_market.current_price() - src_market.current_price());
                return Some(());
            }
            linked_markets.iter()
                .min_by(|(_, a), (_, b)| {
                    a.current_price().cmp(&b.current_price())
                })
                .map(|(lowest_city, lowest_market)| {
                    // no profit to be made by buying, so instead travel to location with lowest price with empty cargo
                    info!("Agent {} moving from {} to {} in order to buy {} at lowest price of {}",
                        agent, src_city, lowest_city, good, lowest_market.current_price());
                    movement.send(Movement {
                        from: pos.clone(),
                        to: GraphPosition::Node(*lowest_city),
                        entity: e_agent,
                    });
                })
        })
        .unwrap_or_else(|| info!("Agent {} cannot move because no linked cities from {}", agent, src_city));

    Ok(())
}


