use bevy::prelude::*;

use crate::world::rocks::{Rock, ROCK_SIZE};

pub struct RocksPlugin;

impl Plugin for RocksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(add_components.system());
    }
}

pub struct RockGraphics;

fn add_components(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rocks: Query<Without<RockGraphics, (Entity, &Rock)>>,
) {
    for (entity, rock) in rocks.iter() {
        println!("{:?}", entity);
        commands
            .insert(
                entity,
                SpriteComponents {
                    material: materials
                        .add(Color::rgb_linear(0.5, 0.5, 1.0).into()),
                    sprite: Sprite::new(ROCK_SIZE.into()),
                    transform: Transform::from_translation(
                        rock.position.extend(0.0),
                    ),
                    ..Default::default()
                },
            )
            .insert_one(entity, RockGraphics);
    }
}
