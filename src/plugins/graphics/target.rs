use bevy::prelude::*;

use crate::world::target::Target;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::update_graphics.system());
    }
}

impl TargetPlugin {
    fn update_graphics(mut targets: Query<&mut Target>) {
        for mut target in targets.iter_mut() {
            if target.has_changed() {
                // TASK: Display target, if selected.
                println!("Target: {:?}", target.position());
            }
        }
    }
}
