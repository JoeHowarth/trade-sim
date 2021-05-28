mod impls;

pub use impls::*;
use crate::{
    prelude::*,
    types::*,
};
use rand::prelude::SmallRng;
use crate::market::exchanger::{MarketInfo, Exchanger};
use crate::market::{Money, LinearMarket};

/*
Agent Components:
- Agent
- GraphPosition
- Cargo
- Money
 */

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct AgentHandle {
    pub agent: Agent,
    pub entity: Entity,
}

#[derive(Debug, From, Clone, Eq, PartialEq, Hash)]
pub struct Agent {
    pub name: String,
}

#[derive(Debug, From, Clone, Eq, PartialEq, Hash)]
pub struct Cargo {
    pub good: Good,
    pub amt: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GraphPosition {
    Node(CityHandle),
    Edge(CityHandle, CityHandle),
}

/*
Functions
 */

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

fn decide_single_good(
    agent: &Agent,
    cargo: &mut Cargo,
    wallet: &mut Money,
    local_market: &mut MarketInfo,
    adj_markets: &HashMap<City, &MarketInfo>,
) {
    let cmp = |(_, l_market): &(&City, &&MarketInfo), (_, r_market): &(&City, &&MarketInfo)| {
        l_market.current_price().cmp(&r_market.current_price())
    };
    let a = adj_markets.iter();
    let dst = adj_markets.iter().max_by(cmp).map(|(city, market)| {
        if market.current_price() <= local_market.current_price() {
            info!("No adjacent markets have higher prices, moving to lowest price market w/o buying");
            return adj_markets.iter().min_by(cmp).expect("should be non-empty").0
        }

        local_market.buy(wallet, 1);
        cargo.amt = 1;
        return city
    });
    match dst {
       Some(city) => {
       }
       None => {},
    };
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
    mut agent_q: Query<(&Agent, &mut Cargo, &mut Money, &GraphPosition)>,
    mut cities_q: Query<(&mut MarketInfo, &LinkedCities), With<City>>,
) -> Result<()> {
    for (agent, cargo, wallet, pos) in agent_q.iter_mut() {
        let city: &CityHandle = pos.city().context("haven't implemented non-city agents yet")?;
        let (_, _) = cities_q
            .get_mut(city.entity)
            .map_err(ecs_err)?;
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




