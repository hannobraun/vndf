use bevy::prelude::*;
use bevy_rapier2d::physics::{
    ColliderHandleComponent, RigidBodyHandleComponent,
};

use crate::world::physics::{ColliderMap, Physics};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ColliderMap::new())
            // Cleanup must come before setup, or it won't work. If setup comes
            // first, the collider is already added to the map, but the queries
            // for the rest of the frame won't yet contain the entity.
            .add_system(Self::cleanup.system())
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

    fn cleanup(
        mut collider_map: ResMut<ColliderMap>,
        entities: Query<Entity, With<Physics>>,
    ) {
        collider_map.retain(|entity| entities.get(entity).is_ok());
    }
}
