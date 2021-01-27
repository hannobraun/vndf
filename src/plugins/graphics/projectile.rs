use bevy::prelude::*;

use crate::{
    graphics::{projectile::ProjectileGraphics, COLOR_PLAYER, LAYER_WORLD},
    world::projectile::Projectile,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::setup.system());
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
                        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
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

    // TASK: Add system that updates transform, setting the z coordinate to
    //       `LAYER_WORLD`.
}
