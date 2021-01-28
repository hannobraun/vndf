use bevy::prelude::*;

use crate::{
    graphics::{projectile::ProjectileGraphics, COLOR_PLAYER, LAYER_WORLD},
    world::projectile::{self, Projectile},
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::setup.system())
            .add_system(Self::update_layer.system());
    }
}

impl ProjectilePlugin {
    fn setup(
        commands: &mut Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        projectiles: Query<
            Entity,
            (With<Projectile>, Without<ProjectileGraphics>),
        >,
    ) {
        for projectile in projectiles.iter() {
            commands
                .insert(
                    projectile,
                    SpriteBundle {
                        material: materials.add(COLOR_PLAYER.into()),
                        sprite: Sprite::new(projectile::SIZE),
                        // TASK: Remove transform. It should be added by the
                        //       physics system.
                        transform: Transform::from_translation(Vec3::new(
                            100.0,
                            0.0,
                            LAYER_WORLD,
                        )),
                        ..Default::default()
                    },
                )
                .insert_one(projectile, ProjectileGraphics);
        }
    }

    fn update_layer(
        mut projectiles: Query<&mut Transform, With<ProjectileGraphics>>,
    ) {
        for mut transform in projectiles.iter_mut() {
            transform.translation.z = LAYER_WORLD;
        }
    }
}
