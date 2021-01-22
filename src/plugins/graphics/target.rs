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
        mut materials: ResMut<Assets<ColorMaterial>>,
        targets: Query<Entity, (With<Target>, Without<TargetGraphics>)>,
    ) {
        for target in targets.iter() {
            let entity = commands
                .spawn(SpriteBundle {
                    material: materials
                        .add(Color::rgb_linear(1.0, 0.0, 0.0).into()),
                    sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                    ..Default::default()
                })
                .current_entity()
                .unwrap();
            commands.insert_one(target, TargetGraphics::new(entity));
        }
    }

    fn update_graphics(mut targets: Query<(&mut Target, &mut TargetGraphics)>) {
        for (mut target, graphics) in targets.iter_mut() {
            if target.has_changed() {
                // TASK: Display target, if selected.
                println!(
                    "Target: {:?} (graphics: {:?})",
                    target.position(),
                    graphics.entity()
                );
            }
        }
    }
}
