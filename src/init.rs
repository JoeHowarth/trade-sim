use bevy::utils::tracing::field::debug;
use structopt::StructOpt;
use warp::Filter;

use types::prelude::*;
use types::agent::{Cargo, GraphPosition};
use types::market::Money;
use types::{CityHandle, City, Goods, Good};
use types::market::exchanger::MarketInfo;


#[derive(StructOpt)]
pub struct Cli {
    file: Option<std::path::PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct Input {
    pub settings: Settings,
    cities: Vec<CityInput>,
    agents: Vec<AgentInput>,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub loop_rate: u64,
}

#[derive(Deserialize, Debug)]
pub struct CityInput {
    name: Ustr,
    links: Vec<Ustr>,
    market: HashMap<Good, MarketInfo>,
    pos: Option<Vec2>,
}

#[derive(Deserialize, Debug)]
struct AgentInput {
    name: Ustr,
    position: AgentPositionInput,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum AgentPositionInput {
    Node(Ustr),
    // Edge(String, String),
}

pub fn get_input() -> Result<Input> {
    let args = Cli::from_args();
    let file: std::path::PathBuf = args.file.unwrap_or("testfile.yml".into());
    let input: Result<Input, _> = serde_yaml::from_reader(
        io::BufReader::new(
            fs::File::open(&file).context("Failed to open input file")?
        ))
        .context("Failed to serialize input");
    match input {
        ok @ Ok(_) => {
            ok
        }
        e @ Err(_) => {
            let reader = io::BufReader::new(fs::File::open(&file)?);
            let json_val: serde_json::Value = serde_yaml::from_reader(reader)?;
            println!("{}", serde_json::to_string_pretty(&json_val)?);
            e.with_context(|| format!("Failed to open input file {:?}", &file))
        }
    }
}

pub fn init(
    input: Res<Input>,
    mut commands: Commands,
) -> Result<()> {
    let goods = Goods(input.cities.iter()
        .flat_map(|c| c.market.keys())
        .cloned()
        .collect());
    let cities_to_handles = init_cities(&mut commands, &input.cities)?;
    init_agents(&mut commands, &input.agents, &cities_to_handles, &goods)?;

    commands.insert_resource(goods);
    Ok(())
}

fn init_agents(
    commands: &mut Commands,
    input_agents: &Vec<AgentInput>,
    cities_to_handles: &HashMap<City, CityHandle>,
    all_goods: &Goods,
) -> Result<()> {
    let mut rng = SmallRng::from_entropy();
    for agent in input_agents.iter() {
        // - Agent - GraphPosition - Cargo - Money
        let graph_pos: GraphPosition = match agent.position {
            AgentPositionInput::Node(city) => {
                let city_handle = cities_to_handles.get(&City { name: city })
                    .context("Agent input has non-existent city")?;
                GraphPosition::Node(*city_handle)
            }
        };
        commands.spawn().insert_bundle((
            types::agent::Agent { name: ustr(&agent.name) },
            graph_pos,
            Cargo {
                good: all_goods.0.iter().choose(&mut rng).unwrap().clone(),
                amt: 1,
            },
            Money(20.),
        ));
    }
    Ok(())
}

pub fn init_cities(
    commands: &mut Commands,
    input_cities: &Vec<CityInput>,
) -> Result<HashMap<City, CityHandle>> {
    let mut thread_rng = SmallRng::from_entropy();
    let cities: Vec<CityHandle> = input_cities.iter().map(|city| {
        let info: City = city.name.into();
        let entity = commands.spawn_bundle((
            info.clone(),
            city.market[&("Grain".into())].clone()
        ))
            .insert(types::GridPosition::from(city.pos
                .unwrap_or_else(|| Vec2::from((
                    thread_rng.gen::<f32>() * 10.,
                    thread_rng.gen::<f32>() * 10., )))))
            .id();

        return CityHandle { entity, city: info };
    })
        .collect();

    let name_to_ch: HashMap<Ustr, &CityHandle> = cities.iter()
        .map(|ch| (ch.city.name, ch, )).collect();

    let links: HashMap<&CityHandle, Vec<CityHandle>> = input_cities.iter()
        .map(|c| (
            name_to_ch[&c.name],
            c.links.iter().flat_map(|l| name_to_ch.get(l).cloned()).cloned().collect(),
        )).collect();

    // validate that every edge is bi-directional
    for (src, dsts) in links.iter() {
        for dst in dsts.iter() {
            let reverse_links = links.get(dst);
            if let Some(reverse_links) = reverse_links {
                if !reverse_links.contains(src) {
                    bail!("Found non-bidirectional edge");
                }
            } else {
                bail!("Found non-bidirectional edge");
            }
        }
    }

    let mut cities_to_entities = HashMap::with_capacity(links.len());
    // add links
    for (src, links) in links.into_iter() {
        commands.entity(src.entity).insert(types::LinkedCities(links));
        cities_to_entities.insert(src.city.clone(), src.clone());
    }
    commands.insert_resource(cities_to_entities.clone());
    Ok(cities_to_entities)
}
