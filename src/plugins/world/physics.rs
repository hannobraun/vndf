use bevy::prelude::*;
use bevy_rapier2d::physics::{
    ColliderHandleComponent, RigidBodyHandleComponent,
};

use crate::world::physics::Physics;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::setup.system());
    }
}

impl PhysicsPlugin {
    fn setup(
        commands: &mut Commands,
        entities: Query<
            Entity,
            (
                With<RigidBodyHandleComponent>,
                With<ColliderHandleComponent>,
                Without<Physics>,
            ),
        >,
    ) {
        for entity in entities.iter() {
            commands.insert_one(entity, Physics);
        }
    }
}
