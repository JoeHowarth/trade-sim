#![allow(unused_imports, dead_code)]

use types::agent::{AgentHandle, Cargo, GraphPosition};
use types::prelude::*;
use types::City;
use types::market::exchanger::{DryRunExchanger, MarketInfo};
use types::market::Money;
use types::Order;
use types::query_like::QueryLike;
use crate::movement::transition_movement;
use crate::order_clearing::{Failed, transition_order};

pub mod agent_behavior;
pub mod order_clearing;
pub mod movement;

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
            let new_location = transition_movement(m, state.location)?;
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
            transition_order(order, &mut market, &mut money, &mut cargo)
                .map(|_| AgentState {
                    cargo,
                    money,
                    ..*state
                })
        }
    }
}

pub(crate) fn setup_tests() -> bevy::app::AppBuilder {
    use bevy::app;
    use bevy::log::LogPlugin;
    use bevy::core::CorePlugin;
    use bevy::diagnostic::DiagnosticsPlugin;

    let mut app = App::build();
    app.insert_resource(app::ScheduleRunnerSettings {
        run_mode: app::RunMode::Once
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
