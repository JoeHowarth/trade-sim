#![allow(unused_imports, dead_code)]
extern crate derive_more;

pub mod types;
pub mod prelude;
mod market;
mod init;
mod web;
mod agent;

use crate::{
    types::*,
    prelude::*,
    agent::{move_agents_random, GraphPosition, Agent, Cargo},
    market::{
        Money,
        exchanger::{MarketInfo}
    },
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
use crate::agent::{agents_sell, agents_buy_random};

#[derive(Debug)]
pub struct State {
    pub tick: Tick,
    pub nodes: Vec<(City, LinkedCities, MarketInfo, GridPosition)>,
    pub agents: Vec<(Agent, GraphPosition, Money, Cargo)>
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = init::get_input().context("Failed to get input")?;
    let (state_tx, state_rx) = watch::channel(
        State {
            tick: Tick(0),
            nodes: Vec::new(),
            agents: Vec::new(),
        }
    );
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
        .add_resource(Tick(0))
        .add_resource(HashMap::<City, Entity>::default())
        .add_plugin(LogPlugin)
        .add_plugin(ReflectPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(ScheduleRunnerPlugin {})
        .add_startup_system(init::init.system().chain(fatal_error_handler_system.system()))
        .add_stage("pre-work", SystemStage::serial()
            .with_system(update_tick.system()))
        .add_stage("main-loop", SystemStage::serial()
            .with_system(update_cities.system())
            .with_system(wrap(agents_sell.system()))
            .with_system(wrap(agents_buy_random.system()))
            .with_system(wrap(move_agents_random.system())))
        .add_stage("final-work", SystemStage::serial()
            .with_system(wrap(printer.system())))
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

fn update_tick(mut tick: ResMut<Tick>) {
    tick.0 += 1;
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

fn printer(
    state_tx: Res<watch::Sender<State>>,
    tick: Res<Tick>,
    cities_q: Query<(&City, &LinkedCities, &MarketInfo, &GridPosition)>,
    agents_q: Query<(&Agent, &GraphPosition, &Money, &Cargo)>,
) -> Result<()> {
    info!("Starting printing combined query");
    let mut state = State {
        tick: Tick(0),
        nodes: Vec::with_capacity(100),
        agents: Vec::with_capacity(100),
    };
    for (city, links, market_info, pos) in cities_q.iter() {
        state.nodes.push((city.clone(), links.clone(), market_info.clone(), *pos));
        // info!("City:          {:?}", city);
        // info!("Market Info:   {:?}", market_info);
        // info!("Current Price: {:?}", market_info.current_price());
        // info!("Links:         {:?}", links);
    }
    for (agent, pos, money, cargo) in agents_q.iter() {
        state.agents.push((agent.clone(), pos.clone(), money.clone(), cargo.clone()))

    }
    state.tick = tick.clone();
    info!("state:         {:?}", state);
    state_tx.send(state).context("Failed to send state to server")
}

