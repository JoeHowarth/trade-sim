#![allow(unused_imports, dead_code)]

use crate::{
    movement::transition_movement,
    order_clearing::{transition_order, Failed},
};
use types::{
    agent::{AgentHandle, Cargo, GraphPosition},
    market::{
        exchanger::{DryRunExchanger, MarketInfo},
        Money,
    },
    prelude::*,
    query_like::QueryLike,
    City, Order,
};

pub mod agent_behavior;
pub mod movement;
pub mod order_clearing;

#[derive(Clone, Debug)]
pub struct AgentState {
    agent: AgentHandle,
    location: GraphPosition,
    cargo: Cargo,
    money: Money,
}

fn transition_state(
    state: &AgentState,
    cities: &dyn QueryLike<(&CityHandle, &MarketInfo, &LinkedCities)>,
    action: &Action,
) -> Result<AgentState> {
    match action {
        Action::Movement(m) => {
            let new_location =
                transition_movement(m, state.location)?;
            Ok(AgentState {
                location: new_location,
                ..*state
            })
        }
        Action::Order(order) => {
            let (_, market, _) = cities.get(order.market.entity)?;
            let mut market = DryRunExchanger { inner: market };
            let mut cargo = state.cargo.clone();
            let mut money = state.money.clone();
            transition_order(
                order,
                &mut market,
                &mut money,
                &mut cargo,
            )
            .map(|_| AgentState {
                cargo,
                money,
                ..*state
            })
        }
    }
}

pub(crate) fn setup_tests() -> bevy::app::App {
    use bevy::{
        app, core::CorePlugin, diagnostic::DiagnosticsPlugin,
        log::LogPlugin,
    };

    let mut app = App::new();
    app.insert_resource(app::ScheduleRunnerSettings {
        run_mode: app::RunMode::Once,
    })
    .insert_resource(types::Tick(0))
    .add_event::<Order>()
    .add_event::<Failed<Order>>()
    .add_plugin(LogPlugin)
    .add_plugin(CorePlugin)
    .add_plugin(DiagnosticsPlugin)
    .add_plugin(app::ScheduleRunnerPlugin {});
    app
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
