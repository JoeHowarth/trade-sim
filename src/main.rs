#![allow(unused_imports, dead_code)]
// #![feature(async_closure)]
extern crate derive_more;

use std::time::Duration;
use tokio::sync::mpsc;
use bevy::{
    log::{LogPlugin, LogSettings},
    app::{RunMode, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    core::CorePlugin,
    diagnostic::DiagnosticsPlugin,
};

use types::{
    *,
    prelude::*,
    market::exchanger::MarketInfo,
    agent::{GraphPosition, Cargo, Agent},
    market::Money,
};
use sim::agent_behavior;
use types::market::exchanger::Order;
use structopt::StructOpt;
use sim::order_clearing::Failed;
use crate::init::Cli;

mod init;

fn main() -> Result<()> {
    let args = Cli::from_args();
    if let Some(serve_static_filename) = args.serve_static {
        return server::static_server(serve_static_filename);
    }

    let input = init::get_input(args).context("Failed to get input")?;
    let (state_tx, state_rx) = mpsc::unbounded_channel();
    let (save_signal_sender, save_signal_recv) = mpsc::unbounded_channel();
    let mut app = build_app(input, state_tx, save_signal_sender);

    server::spawn(state_rx, save_signal_recv);

    app.run();
    Ok(())
}

fn build_app(
    input: init::Input,
    state_tx: mpsc::UnboundedSender<types::State>,
    _save_signal_sender: mpsc::UnboundedSender<Option<String>>, // TODO save on key press
) -> AppBuilder {
    let mut app = App::build();
    app
        .insert_resource(LogSettings {
            level: bevy::log::Level::DEBUG,
            filter: "bevy_ecs=info,bevy_app=info,bevy_core=info".into(),
        })
        .insert_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(Duration::from_millis(input.settings.loop_rate.clone()))
            }
        })
        .insert_resource(input)
        .insert_resource(state_tx)
        .insert_resource(Tick(0))
        .insert_resource(HashMap::<City, Entity>::default())
        .add_event::<Order>()
        .add_event::<Movement>()
        .add_event::<Failed<Order>>()
        .add_event::<Failed<Movement>>()
        .add_plugin(LogPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(ScheduleRunnerPlugin {})
        .add_startup_system(wrap(init::init.system()))
        .add_stage("pre-work",
                   SystemStage::single_threaded()
                       .with_system(update_tick.system())
                       .with_system(update_cities.system()))
        .add_stage("decision-stage",
                   SystemStage::single_threaded()
                       .with_system(wrap(agent_behavior::decide.system())))
        .add_stage("action-stage",
                   SystemStage::single_threaded()
                       .with_system(sim::movement::movement.system())
                       .with_system(sim::order_clearing::clear_orders.system()))
        .add_stage("final-work",
                   SystemStage::single_threaded()
                       .with_system(wrap(printer.system())));
    app
}

fn wrap<T: System<In=(), Out=Result<()>>>(inner: T) -> impl System<In=(), Out=()> {
    inner.chain(error_handler_system::<T>.system())
}

fn error_handler_system<T: System>(In(result): In<Result<()>>) {
    if let Err(err) = result {
        error!("Error from system {}:\n{}", std::any::type_name::<T>(), err);
    }
}

fn fatal_error_handler_system(In(result): In<Result<()>>) {
    if let Err(err) = result {
        error!("{:?}", err);
        std::process::abort()
    }
}

fn update_tick(mut tick: ResMut<Tick>, mut has_run: Local<bool>) {
    if !*has_run {
        *has_run = true;
        return;
    }
    tick.0 += 1;
}

fn update_cities(mut q: Query<(&City, &mut MarketInfo)>) {
    for (_city, mut market_info) in q.iter_mut() {
        info!("Supply in {} is {}", _city, market_info.supply);
        market_info.produce_and_consume();
    }
}

fn printer(
    state_tx: Res<mpsc::UnboundedSender<types::State>>,
    tick: Res<Tick>,
    cities_q: Query<(&City, &LinkedCities, &MarketInfo, &GridPosition)>,
    agents_q: Query<(&Agent, &GraphPosition, &Money, &Cargo)>,
) -> Result<()> {
    // info!("Starting printing combined query");
    let mut state = types::State {
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
    // info!("state:         {:?}", state);
    state_tx.send(state)?;
    Ok(())
}

