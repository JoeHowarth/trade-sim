use types::prelude::*;
use types::agent::{Agent, Cargo, GraphPosition};
use types::market::{Money, Market};
use types::market::exchanger::{MarketInfo, Exchanger};
use types::{City, CityHandle, ecs_err, LinkedCities};
use std::cmp::Ordering;

fn sell_cargo(
    cargo: &mut Cargo,
    wallet: &mut Money,
    market: &mut MarketInfo,
) {
    let amt = cargo.amt.clone();
    market.sell(wallet, amt as i32);
    cargo.amt = 0;
}

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

fn decide_single_good<'a>(
    agent: &Agent,
    cargo: &mut Cargo,
    wallet: &mut Money,
    pos: &mut GraphPosition,
    city_to_market: &HashMap<&CityHandle, &MarketInfo>,
    city_to_links: &HashMap<CityHandle, &LinkedCities>,
)  {
    // let (hm, m): = city_to_market.iter()
    //     .partition(|m| m.0 == city);
    // let local_market = m.next().unwrap();
    let city = pos.city().unwrap();
    let links = city_to_links[city];
    let dst: Option<&CityHandle> = links.iter()
        .map(|ch| (ch, city_to_market[ch]))
        .max_by(|(_, a), (_, b)| a.current_price().cmp(&b.current_price()))
        .map(|(city, market)| {
            let local_market = city_to_market[city];
            if market.current_price() <= local_market.current_price() {
                info!("No adjacent markets have higher prices, moving to lowest price market w/o buying");
                links.iter()
                    .map(|ch| (ch, city_to_market[ch]))
                    .max_by(|(_, a), (_, b)| a.current_price().cmp(&b.current_price()))
                    .expect("should be non-empty").0;
            }

            // |local_market| local_market.buy(wallet, 1);
            cargo.amt = 1;
            return city;
        });
    // match dst {
    //     Some(city) => {
    //         *pos = GraphPosition::Node(city.clone())
    //         return |m| m.
    //     },
    //     None => {}
    // };

}

/*
Systems

Order:
- sell inventory
- decide where to go next and buy good
- move
 */

pub fn agents_sell(
    mut agent_q: Query<(&mut Cargo, &mut Money, &GraphPosition), With<Agent>>,
    mut cities_q: Query<(&City, &mut MarketInfo)>,
) -> Result<()> {
    for (mut cargo, mut wallet, pos) in agent_q.iter_mut() {
        let city: &CityHandle = pos.city().context("haven't implemented non-city agents yet")?;
        let mut market: Mut<MarketInfo> = cities_q
            .get_component_mut(city.entity)
            .map_err(ecs_err)?;

        sell_cargo(&mut cargo, &mut wallet, &mut market);
    }
    Ok(())
}

pub fn agents_move_single_good(
    mut agent_q: Query<(&Agent, &mut Cargo, &mut Money, &mut GraphPosition)>,
    mut cities_q: Query<(Entity, &City, &mut MarketInfo, &LinkedCities)>,
) -> Result<()> {
    let mut city_to_links = HashMap::new();
    let mut city_to_market: HashMap<CityHandle, Mut<MarketInfo>> = HashMap::from_iter(
        cities_q.iter_mut().map(|(entity, city, mut market_info, linked_cities): (Entity, &City, Mut<MarketInfo>, &LinkedCities)| {
            let handle = CityHandle { entity, city: city.clone() };
            city_to_links.insert(handle.clone(), linked_cities);
            (handle, market_info)
        }));
    for (agent, mut cargo, mut wallet, mut pos) in agent_q.iter_mut() {
        let city: CityHandle = pos.city().context("haven't implemented non-city agents yet")?.clone();
        let city_to_market_static = city_to_market.iter().map(|(k, v)| (k, v.deref())).collect::<HashMap<&CityHandle, &MarketInfo>>();
        decide_single_good(agent, &mut cargo, &mut wallet, &mut pos,  &city_to_market_static, &city_to_links)
    }
    Ok(())
}

pub fn agents_buy_random(
    mut agent_q: Query<(&Agent, &mut Cargo, &mut Money, &GraphPosition)>,
    mut cities_q: Query<&mut MarketInfo, With<City>>,
) -> Result<()> {
    for (agent, mut cargo, mut wallet, pos) in agent_q.iter_mut() {
        let city: &CityHandle = pos.city().context("haven't implemented non-city agents yet")?;
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




