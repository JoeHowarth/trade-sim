#![allow(unused_imports, dead_code)]
use types::prelude::*;
use types::City;
use types::Order;
use crate::order_clearing::Failed;

pub mod agent_behavior;
pub mod order_clearing;

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
