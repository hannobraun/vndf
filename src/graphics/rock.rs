use bevy::prelude::*;

use crate::world::rock::Rock;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::setup.system());
    }
}

impl RockPlugin {
    fn setup(
        commands: &mut Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        rocks: Query<(Entity, &Rock), Without<RockGraphics>>,
    ) {
        for (entity, rock) in rocks.iter() {
            commands
                .insert(
                    entity,
                    SpriteBundle {
                        material: materials
                            .add(Color::rgb_linear(0.5, 0.5, 1.0).into()),
                        sprite: Sprite::new(Vec2::new(
                            rock.size(),
                            rock.size(),
                        )),
                        ..Default::default()
                    },
                )
                .insert_one(entity, RockGraphics);
        }
    }
}

pub struct RockGraphics;
