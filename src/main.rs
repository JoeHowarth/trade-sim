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
    let state = Arc::new(Mutex::new(0));
    {
        let other_state = state.clone();
        std::thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(server(other_state));
        });
    }
    App::build()
        .add_resource(LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "bevy_ecs=info,bevy_app=info,bevy_core=info".into(),
        })
        .add_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {wait: Some(Duration::from_millis(1000))}
        })
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

async fn server(state: Arc<Mutex<i32>>) {
    // Match any request and return hello world!
    let routes = warp::any()
        .map(move || format!("Count is: {:?}", state.lock()));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
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

fn printer(state: Res<Arc<Mutex<i32>>>,q: Query<(&City, &LinkedCities, &MarketInfo)>) {
    info!("Starting printing combined query");
    let mut state = state.lock().unwrap();
    info!("state:         {:?}", state);
    state.add_assign(1);
    for (city, links, market_info) in q.iter() {
        info!("City:          {:?}", city);
        info!("Market Info:   {:?}", market_info);
        info!("Current Price: {:?}", market_info.current_price());
        info!("Links:         {:?}", links);
    }
}

