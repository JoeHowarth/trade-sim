#![allow(unused_imports, dead_code)]
extern crate derive_more;

pub mod types;
pub mod prelude;

use crate::{
    types::*,
    prelude::*,
};

fn main() {
    App::build()
        .add_startup_system(init.system())
        .add_system(greet.system())
        .run();
}

fn init(commands: &mut Commands) {
    let city_names = &["A", "B", "C", "D"];
    let adjacent: &[(&str, &[&str])] = &[
        ("A", &["B", "C"]),
        ("B", &["A", "D"]),
        ("C", &["A", "D", "A"]),
        ("D", &["B", "C"]),
    ];
    let mut thread_rng = rand::thread_rng();
    let cities: Vec<_> = city_names.iter().map(|city_name| {
        let info = City { name: city_name.to_string() };
        let entity = commands
            .spawn((info.clone(), ))
            .with(Position::new(thread_rng.gen::<(f32, f32)>()))
            .current_entity().unwrap();
        CityHandle { entity, info }
    }).collect();

    let name_to_ch: HashMap<String, CityHandle> = HashMap::from_iter(
        cities.iter()
            .map(|ch| {
                (ch.info.name.clone(), ch.clone())
            }));
    let mut ch_to_links = name_to_ch.iter()
        .map(|(_, ch)| (ch.clone(), Vec::new()))
        .collect::<HashMap<CityHandle, Vec<CityHandle>>>();

    for (src, dst) in adjacent.iter() {
        let src_ch = name_to_ch.get(*src).unwrap().clone();
        let links = dst.iter()
            .filter_map(|s| name_to_ch.get(*s).clone())
            .collect();
        ch_to_links.insert(src_ch, links);

        // let src_ch = cities.iter().filter(|ch| &ch.name == src).next();
        // commands
        //     .set_current_entity(src_ch.unwrap().entity);
        // commands.with(LinkedCities(dst.iter()
        //     .filter_map(|s| cities.iter()
        //         .filter(|c| &c.name == s)
        //         .next()
        //         .map(|ch| CityHandle::clone(ch))
        //     )
        //     .collect::<Vec<_>>()));
    };
}


fn greet() {
    println!("hi system, this is Joe 3!");
}

