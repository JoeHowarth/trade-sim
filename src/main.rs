#![allow(unused_imports, dead_code)]
extern crate derive_more;

pub mod types;
pub mod prelude;
mod market;

use crate::{
    types::*,
    prelude::*,
};
use bevy::log::{LogPlugin, LogSettings};

fn main() {
    App::build()
        .add_resource(LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "bevy_ecs=info".into(),
        })
        .add_plugin(LogPlugin)
        .add_startup_system(init.system())
        .add_system(printer.system())
        .run();
}

fn printer(q: Query<(&City, &LinkedCities)>) {
    info!("Starting printing combined query");
    for (city, links) in q.iter() {
        info!("City: {:?}", city);
        info!("Links: {:?}\n", links);
    }
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
    let mut ch_to_links: HashMap<CityHandle, Vec<CityHandle>> = name_to_ch.iter()
        .map(|(_, ch)| (ch.clone(), Vec::new()))
        .collect();

    for (src, dst) in adjacent.iter() {
        ch_to_links.insert(
            name_to_ch.get(*src).unwrap().clone(),
            dst.iter()
                .filter_map(|&s| name_to_ch.get(s))
                .cloned()
                .collect(),
        );
    };
    dbg!("init");

    // validate that every edge is bi-directional
    for (src, dsts) in ch_to_links.iter() {
        for dst in dsts.iter() {
            let reverse_links = ch_to_links.get(dst);
            assert!(reverse_links.is_some());

            if let Some(reverse_links) = reverse_links {
                assert!(reverse_links.contains(src));
                dbg!("validation");
            }
        }
    }

    // add links
    for (src, links) in ch_to_links.drain() {
        dbg!((&src, &links));
        commands.insert(src.entity, (src.info, LinkedCities(links), ));
    }
}



