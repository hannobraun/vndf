use bevy::prelude::*;

use crate::world::rock::Rock;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(setup.system());
    }
}

pub struct RockGraphics;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rocks: Query<Without<RockGraphics, (Entity, &Rock)>>,
) {
    for (entity, rock) in rocks.iter() {
        commands
            .insert(
                entity,
                SpriteComponents {
                    material: materials
                        .add(Color::rgb_linear(0.5, 0.5, 1.0).into()),
                    sprite: Sprite::new(Vec2::new(rock.size(), rock.size())),
                    ..Default::default()
                },
            )
            .insert_one(entity, RockGraphics);
    }
}
