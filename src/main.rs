#![allow(unused_imports, dead_code)]
extern crate derive_more;

pub mod types;
pub mod prelude;
mod market;
mod init;

use crate::{
    types::*,
    prelude::*,
    market::exchanger::{MarketInfo},
};
use bevy::{
    log::{LogPlugin, LogSettings},
    reflect::ReflectPlugin,
};
use bevy::core::CorePlugin;
use bevy::diagnostic::DiagnosticsPlugin;

fn main() -> Result<(), Box<dyn Error>> {
    let input = init::get_input()?;
    App::build()
        .add_resource(LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "bevy_ecs=info".into(),
        })
        .add_resource(input)
        .add_plugin(LogPlugin)
        .add_plugin(ReflectPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_startup_system(init::init.system())
        .add_system(update_cities.system())
        .add_system(printer.system())
        .run();
    Ok(())
}

fn update_cities(mut q: Query<(&City, &mut MarketInfo)>) {
    for (_city, mut market_info) in q.iter_mut() {
        market_info.produce_and_consume();
    }
}

impl MarketInfo {
    pub fn produce_and_consume(&mut self) {
        self.supply += self.production - self.consumption
    }
}

fn printer(q: Query<(&City, &LinkedCities, &MarketInfo)>) {
    info!("Starting printing combined query");
    for (city, links, market_info) in q.iter() {
        info!("City:        {:?}", city);
        info!("Market Info: {:?}", market_info);
        info!("Links:       {:?}\n", links);
    }
}

