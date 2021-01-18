use bevy::prelude::*;

use crate::world::{ship::Ship, target::Target};

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::add_components.system());
    }
}

impl TargetPlugin {
    fn add_components(
        commands: &mut Commands,
        ships: Query<Entity, (With<Ship>, Without<Target>)>,
    ) {
        for ship in ships.iter() {
            commands.insert_one(ship, Target::new());
        }
    }
}
