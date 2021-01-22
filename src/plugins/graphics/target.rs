use bevy::prelude::*;

use crate::{
    graphics::{target::TargetGraphics, LAYER_UI},
    world::target::Target,
};

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
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                })
                .current_entity()
                .unwrap();
            commands.insert_one(target, TargetGraphics::new(entity));
        }
    }

    fn update_graphics(
        mut ships: Query<(&mut Target, &mut TargetGraphics)>,
        mut targets: Query<&mut Transform>,
    ) {
        for (mut target, target_graphics) in ships.iter_mut() {
            if target.has_changed() {
                let mut transform =
                    targets.get_mut(target_graphics.entity()).unwrap();
                match target.position() {
                    Some(position) => {
                        *transform = Transform::from_translation(
                            position.extend(LAYER_UI),
                        );
                    }
                    None => {
                        // TASK: Hide target (check out `Visible` component).
                        println!("Target clear");
                    }
                }
            }
        }
    }
}
