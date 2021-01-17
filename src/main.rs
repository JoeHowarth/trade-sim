#![allow(unused_imports, dead_code)]
extern crate derive_more;

pub mod types;
pub mod prelude;
mod market;
mod init;
mod web;

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

#[derive(Debug)]
pub struct State (pub Vec<(City, LinkedCities, MarketInfo)>);

fn main() -> Result<(), Box<dyn Error>> {
    let input = init::get_input().context("Failed to get input")?;
    let state = Arc::new(Mutex::new(State(vec![])));
    {
        let other_state = state.clone();
        std::thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(web::server(other_state));
        });
    }
    App::build()
        .add_resource(LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "bevy_ecs=info,bevy_app=info,bevy_core=info".into(),
        })
        .add_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(Duration::from_millis(input.settings.loop_rate))
            }
        })
        .add_resource(input)
        .add_resource(state)
        .add_plugin(LogPlugin)
        .add_plugin(ReflectPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(ScheduleRunnerPlugin{})
        .add_startup_system(init::init.system().chain(fatal_error_handler_system.system()))
        .add_system(update_cities.system())
        .add_system(printer.system())
        .run();
    Ok(())
}

use warp::Filter;
use std::ops::{Add, AddAssign};

fn error_handler_system(In(result): In<Result<()>>) {
    if let Err(err) = result {
        error!("{:?}", err);
    }
}

fn fatal_error_handler_system(In(result): In<Result<()>>) {
    if let Err(err) = result {
        error!("{:?}", err);
        std::process::abort()
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

fn printer(state: Res<Arc<Mutex<State>>>, q: Query<(&City, &LinkedCities, &MarketInfo)>) {
    info!("Starting printing combined query");
    let mut state = state.lock().unwrap();
    state.0.clear();
    for (city, links, market_info) in q.iter() {
        state.0.push((city.clone(), links.clone(), market_info.clone()));
        info!("City:          {:?}", city);
        info!("Market Info:   {:?}", market_info);
        info!("Current Price: {:?}", market_info.current_price());
        info!("Links:         {:?}", links);
    }
    info!("state:         {:?}", state);
}

