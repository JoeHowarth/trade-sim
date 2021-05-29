use types::prelude::*;
use types::agent::{Agent, Cargo, GraphPosition, AgentHandle};
use types::market::{Money, Market};
use types::market::exchanger::{MarketInfo, Exchanger, Order};
use types::{City, CityHandle, ecs_err, LinkedCities};
use std::cmp::Ordering;
use types::prelude::hash_map::RandomState;

fn buy_random(
    agent: &Agent,
    cargo: &mut Cargo,
    wallet: &mut Money,
    market_info: &mut MarketInfo,
    // market: &mut LinearMarket,
) {
    let mut rng = SmallRng::from_entropy();
    // match market.table.iter_mut()
    //     .map(|x| Some(x)).chain(std::iter::once(None))
    //     .choose(&mut rng) {
    //     None | Some(None) => { cargo.amt = 0 }
    //     Some(Some((good, market_info))) => {
    //         if let Some(_) = market_info.buy(wallet, 1) {
    //             cargo.good = good.clone();
    //             cargo.amt = 1;
    //         } else {
    //             info!("agent {} doesn't have enough money to buy {}", &agent.name, &good.name)
    //         }
    //     }
    // }
    if rng.gen_bool(0.5) {
        if let Some(_) = market_info.buy(wallet, 1) {
            cargo.amt = 1;
        } else {
            info!("agent {} doesn't have enough money to buy {}", &agent.name, &cargo.good.name);
        }
    }
}

fn decide_single_good(
    agent: AgentHandle,
    wallet: &Money,
    pos: &mut GraphPosition,
    city_to_market: &HashMap<CityHandle, &MarketInfo>,
    city_to_links: &HashMap<CityHandle, &LinkedCities>,
) -> Option<Order> {
    let good = types::Good::from("Grain");

    let current_city = pos.city().unwrap();
    let links = city_to_links[&current_city];
    let max = links.iter()
        .map(|ch| (ch, city_to_market[ch]))
        .max_by(|(_, a), (_, b)| a.current_price().cmp(&b.current_price()));
    match max {
        Some((maybe_dst, market)) => {
            let local_market = city_to_market[maybe_dst];
            if market.current_price() <= local_market.current_price() {
                info!("No adjacent markets have higher prices, moving to lowest price market w/o buying");
                let cheap_dst = links.iter()
                    .map(|ch| (ch, city_to_market[ch]))
                    .max_by(|(_, a), (_, b)| a.current_price().cmp(&b.current_price()))
                    .expect("should be non-empty").0;
                *pos = GraphPosition::Node(*cheap_dst);
            }

            *pos = GraphPosition::Node(*maybe_dst);
            Some(Order { // buy goods in current city to sell at a profit at destination
                good,
                market: current_city,
                agent,
                amt: 1,
            })
        }
        None => None
    }
}

/*
Systems

Order:
- sell inventory
- decide where to go next and buy good
- move
 */

pub fn agents_sell(
    mut agent_q: Query<(Entity, &Agent, &mut Cargo, &GraphPosition)>,
    mut orders: EventWriter<Order>,
) -> Result<()> {
    for (agent_entity, agent, mut cargo, pos) in agent_q.iter_mut() {
        orders.send(Order {
            good: cargo.good,
            market: pos.city().context("haven't implemented non-city agents yet")?,
            agent: AgentHandle { agent: *agent, entity: agent_entity },
            amt: -(cargo.amt as i32),
        });
        cargo.amt = 0
    }
    Ok(())
}

pub fn agents_move_single_good(
    mut agent_q: Query<(Entity, &Agent, &Money, &mut GraphPosition)>,
    cities_q: Query<(Entity, &City, &MarketInfo, &LinkedCities)>,
    mut orders: EventWriter<Order>,
) -> Result<()> {
    let mut city_to_links: HashMap<CityHandle, &LinkedCities> = HashMap::with_capacity(20);
    let mut city_to_market = HashMap::with_capacity(20);
    cities_q.for_each(|(entity, &city, market, links)| {
        let ch = CityHandle { entity, city };
        city_to_links.insert(ch, links);
        city_to_market.insert(ch, market);
    });

    for (entity, &agent, wallet, mut pos) in agent_q.iter_mut() {
        let city: CityHandle = pos.city().context("haven't implemented non-city agents yet")?.clone();
        let agent = AgentHandle { agent, entity };


        if let Some(order) = decide_single_good(agent, &wallet, &mut pos, &city_to_market, &city_to_links) {
            orders.send(order);
        }
    }
    Ok(())
}

pub fn agents_buy_random(
    mut agent_q: Query<(&Agent, &mut Cargo, &mut Money, &GraphPosition)>,
    mut cities_q: Query<&mut MarketInfo, With<City>>,
) -> Result<()> {
    for (agent, mut cargo, mut wallet, pos) in agent_q.iter_mut() {
        let city: CityHandle = pos.city().context("haven't implemented non-city agents yet")?;
        let mut market: Mut<MarketInfo> = cities_q
            .get_component_mut(city.entity)
            .map_err(ecs_err)?;

        buy_random(agent, &mut cargo, &mut wallet, &mut market);
    }
    Ok(())
}

pub fn move_agents_random(
    mut agent_q: Query<&mut GraphPosition, With<Agent>>,
    cities_q: Query<&LinkedCities, With<City>>,
) -> Result<()> {
    let mut rng = SmallRng::from_entropy();
    for mut pos in agent_q.iter_mut() {
        let city = pos.city()
            .context("haven't implemented non-city agents yet")?;
        let links: &LinkedCities = cities_q
            .get_component(city.entity)
            .map_err(ecs_err)?;

        if let Some(linked_city) = links.0.iter().choose(&mut rng) {
            *pos = GraphPosition::Node(linked_city.clone());
        }
    }
    Ok(())
}




