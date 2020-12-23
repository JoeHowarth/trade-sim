use structopt::StructOpt;
use crate::{
    types::*,
    prelude::*,
    market::exchanger::MarketInfo,
};

#[derive(StructOpt)]
pub struct Cli {
    file: Option<std::path::PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct Input {
    cities: Vec<CityInput>
}

#[derive(Deserialize, Debug)]
pub struct CityInput {
    name: String,
    links: Vec<String>,
    market_info: MarketInfo,
    pos: Option<Vec2>,
}

pub fn get_input() -> Result<Input, Box<dyn Error>> {
    let args = Cli::from_args();
    let file: std::path::PathBuf = args.file.unwrap_or("testfile.yml".into());
    let input: Input = serde_yaml::from_reader(io::BufReader::new(fs::File::open(file)?))?;
    Ok(input)
}


pub fn init(
    commands: &mut Commands,
    input: Res<Input>,
) {
    init_cities(commands, &input.cities)
}

pub fn init_cities(
    commands: &mut Commands,
    input_cities: &Vec<CityInput>,
) {
    let mut thread_rng = rand::thread_rng();
    let cities: Vec<CityHandle> = input_cities.iter().map(|city| {
        let info: City = city.name.clone().into();
        let entity = commands.spawn((info.clone(), city.market_info.clone()))
            .with(Position::from(city.pos
                .unwrap_or_else(|| Vec2::from((
                    thread_rng.gen::<f32>() * 10.,
                    thread_rng.gen::<f32>() * 10., )))))
            .current_entity().expect("Failed to create entity");
        CityHandle { entity, info }
    })
        .collect();

    let name_to_ch: HashMap<&String, &CityHandle> = cities.iter()
        .map(|ch| (&ch.info.name, ch, )).collect();

    let links: HashMap<&CityHandle, Vec<CityHandle>> = input_cities.iter()
        .map(|c| (
            name_to_ch[&c.name],
            c.links.iter().map(|l| name_to_ch[l].clone()).collect(),
        )).collect();

    // validate that every edge is bi-directional
    for (src, dsts) in links.iter() {
        for dst in dsts.iter() {
            let reverse_links = links.get(dst);
            assert!(reverse_links.is_some());

            if let Some(reverse_links) = reverse_links {
                assert!(reverse_links.contains(src));
            }
        }
    }

    // add links
    for (src, links) in links.into_iter() {
        commands.insert(src.entity, (LinkedCities(links), ));
    }
}
