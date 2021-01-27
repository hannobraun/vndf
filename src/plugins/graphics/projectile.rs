use bevy::prelude::*;

use crate::graphics::{
    projectile::ProjectileGraphics, COLOR_PLAYER, LAYER_WORLD,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system());
    }
}

impl ProjectilePlugin {
    fn setup(
        commands: &mut Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(SpriteBundle {
                material: materials.add(COLOR_PLAYER.into()),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: Transform::from_translation(Vec3::new(
                    100.0,
                    0.0,
                    LAYER_WORLD,
                )),
                ..Default::default()
            })
            .with(ProjectileGraphics);
    }
}
