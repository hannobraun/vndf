use bevy::prelude::*;
use bevy_rapier2d::physics::{
    ColliderHandleComponent, RigidBodyHandleComponent,
};

use crate::world::physics::{ColliderMap, Physics};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ColliderMap::new())
            .add_system(Self::setup.system());
    }
}

impl PhysicsPlugin {
    fn setup(
        commands: &mut Commands,
        mut collider_map: ResMut<ColliderMap>,
        entities: Query<
            (Entity, &ColliderHandleComponent),
            (With<RigidBodyHandleComponent>, Without<Physics>),
        >,
    ) {
        for (entity, collider) in entities.iter() {
            commands.insert_one(entity, Physics);
            collider_map.insert(collider.handle(), entity);
        }
    }

    // TASK: Clean up collider handles whose entity is removed.
}
