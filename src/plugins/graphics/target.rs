use bevy::prelude::*;

use crate::world::target::Target;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::add_components.system());
    }
}

impl TargetPlugin {
    fn add_components(mut targets: Query<&mut Target>) {
        for mut target in targets.iter_mut() {
            if target.has_changed() {
                println!("Target: {:?}", target.position());
            }
        }
    }
}
