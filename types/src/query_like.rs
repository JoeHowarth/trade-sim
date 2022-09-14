use crate::prelude::*;
use bevy::ecs::{
    component::Component,
    query::{Fetch, QueryEntityError, ReadOnlyFetch, WorldQuery},
    system::QueryComponentError,
};

pub trait QueryLike<'w, W: WorldQuery> {
    fn get(
        &self,
        entity: Entity,
    ) -> Result<<W::ReadOnlyFetch as Fetch>::Item, QueryEntityError>
    where
        <W as WorldQuery>::Fetch: ReadOnlyFetch;

    fn get_mut(
        &mut self,
        entity: Entity,
    ) -> Result<<W::Fetch as Fetch>::Item, QueryEntityError>;
    // where <W as WorldQuery>::Fetch: ;

    // fn get_component<T: Component>(&'w self, entity: Entity) -> Result<&T, QueryComponentError>;
}

impl<'w, 's, Q: WorldQuery> QueryLike<'w, Q> for Query<'w, 's, Q> {
    fn get(
        &self,
        entity: Entity,
    ) -> Result<<Q::ReadOnlyFetch as Fetch>::Item, QueryEntityError>
    where
        <Q as WorldQuery>::Fetch: ReadOnlyFetch,
    {
        self.get(entity)
    }

    fn get_mut(
        &mut self,
        entity: Entity,
    ) -> Result<<Q::Fetch as Fetch>::Item, QueryEntityError> {
        self.get_mut(entity)
    }

    // fn get_component<T: Component>(&'w self, entity: Entity) -> Result<&'w T, QueryComponentError> {
    //     self.get_component::<T>(entity)
    // }
}

mod test {
    use super::*;
    use bevy::{
        core::CorePlugin, diagnostic::DiagnosticsPlugin,
        log::LogPlugin,
    };

    #[test]
    fn test1() {
        let mut app = App::build();
        fn setup(mut cmds: Commands) {
            cmds.spawn_bundle((Tick(2), 43 as usize));
        }
        fn load(q: Query<(Entity, &Tick)>) {
            other(&q);
        }
        fn other(q: &Query<(Entity, &Tick)>) {
            for (e, _) in q.iter() {
                use_query_like(q, e);
            }
        }
        app.insert_resource(app::ScheduleRunnerSettings {
            run_mode: app::RunMode::Once,
        })
        .insert_resource(crate::Tick(0))
        .add_plugin(LogPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(app::ScheduleRunnerPlugin {})
        .add_startup_system(setup.system())
        .add_system(load.system());
        app.run();
    }

    fn use_query_like<'w, 'a>(
        q: &'a impl QueryLike<'a, (Entity, &'w Tick)>,
        e: Entity,
    ) {
        let x = q.get(e);
        match x {
            Ok((_, &Tick(n))) => assert_eq!(n, 2),
            Err(e) => assert!(false, "{}", e),
        }
    }
}
