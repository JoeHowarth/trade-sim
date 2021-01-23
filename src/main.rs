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
use tokio::sync::watch;

#[derive(Debug)]
pub struct State (pub Vec<(City, LinkedCities, MarketInfo, Position)>);

fn main() -> Result<(), Box<dyn Error>> {
    let input = init::get_input().context("Failed to get input")?;
    let (state_tx, state_rx) = watch::channel(State(Vec::new()));
    {
        std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(web::server(state_rx));
        });
    }
    App::build()
        .add_resource(LogSettings {
            level: bevy::log::Level::DEBUG,
            filter: "bevy_ecs=info,bevy_app=info,bevy_core=info".into(),
        })
        .add_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(Duration::from_millis(input.settings.loop_rate))
            }
        })
        .add_resource(input)
        .add_resource(state_tx)
        .add_plugin(LogPlugin)
        .add_plugin(ReflectPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(ScheduleRunnerPlugin{})
        .add_startup_system(init::init.system().chain(fatal_error_handler_system.system()))
        .add_system(update_cities.system())
        .add_system(wrap(printer.system()))
        .run();
    Ok(())
}

fn wrap<T: System<In=(), Out=Result<()>>>(inner: T) -> impl System<In=(), Out=()> {
    inner.chain(error_handler_system.system())
}

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

fn printer(state_tx: Res<watch::Sender<State>>, q: Query<(&City, &LinkedCities, &MarketInfo, &Position)>) -> Result<()>{
    info!("Starting printing combined query");
    let mut state = State(Vec::with_capacity(100));
    for (city, links, market_info, pos) in q.iter() {
        state.0.push((city.clone(), links.clone(), market_info.clone(), *pos));
        info!("City:          {:?}", city);
        info!("Market Info:   {:?}", market_info);
        info!("Current Price: {:?}", market_info.current_price());
        info!("Links:         {:?}", links);
    }
    info!("state:         {:?}", state);
    state_tx.send(state).context("Failed to send state to server")
}

