mod impls;

pub use impls::*;
use crate::{
    prelude::*,
    types::*,
};
use rand::prelude::SmallRng;

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
Systems
 */

pub fn move_agents(
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




