use bevy::prelude::*;

use crate::{graphics::target::TargetGraphics, world::target::Target};

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::add_components.system())
            .add_system(Self::update_graphics.system());
    }
}

impl TargetPlugin {
    fn add_components(
        commands: &mut Commands,
        targets: Query<Entity, (With<Target>, Without<TargetGraphics>)>,
    ) {
        for target in targets.iter() {
            commands.insert_one(target, TargetGraphics);
        }
    }

    fn update_graphics(mut targets: Query<(&mut Target, &mut TargetGraphics)>) {
        for (mut target, graphics) in targets.iter_mut() {
            if target.has_changed() {
                // TASK: Display target, if selected.
                println!(
                    "Target: {:?} (graphics: {:?})",
                    target.position(),
                    graphics
                );
            }
        }
    }
}
