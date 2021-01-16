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
use bevy::app::{ScheduleRunnerPlugin, ScheduleRunnerSettings, RunMode};
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    App::build()
        .add_resource(LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "bevy_ecs=info,bevy_app=info,bevy_core=info".into(),
        })
        .add_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {wait: Some(Duration::from_millis(1000))}
        })
        .add_plugin(LogPlugin)
        .add_plugin(ReflectPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(ScheduleRunnerPlugin{})
        .add_startup_system(init::init.system().chain(error_handler_system.system()))
        .add_system(update_cities.system())
        .add_system(printer.system())
        .run();
    Ok(())
}

fn error_handler_system(In(result): In<Result<()>>) {
    if let Err(err) = result {
        error!("{:?}", err);
    }
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
        info!(Price=market_info.current_price(), "Current Price");
        info!("Links:       {:?}\n", links);
    }
}

