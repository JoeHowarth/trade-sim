#![allow(unused_imports, dead_code)]
extern crate derive_more;

pub mod types;
pub mod prelude;
mod market;
mod init;

use crate::{
    types::*,
    prelude::*,
};
use bevy::log::{LogPlugin, LogSettings};

fn main() -> Result<(), Box<dyn Error>> {
    let input = init::get_input()?;
    App::build()
        .add_resource(LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "bevy_ecs=info".into(),
        })
        .add_resource(input)
        .add_plugin(LogPlugin)
        .add_startup_system(init::init.system())
        .add_system(printer.system())
        .run();
    Ok(())
}

fn printer(q: Query<(&City, &LinkedCities)>) {
    info!("Starting printing combined query");
    for (city, links) in q.iter() {
        info!("City: {:?}", city);
        info!("Links: {:?}\n", links);
    }
}

