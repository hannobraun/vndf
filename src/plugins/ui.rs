use bevy::{diagnostic::Diagnostics, prelude::*};

use crate::{
    ui::{text_bundle, FrameTime, Projectiles, Rocks},
    world::{projectile::Projectile, rock::Rock},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system())
            .add_system(Self::update_frame_time.system())
            .add_system(Self::update_rocks.system())
            .add_system(Self::update_projectiles.system());
    }
}

impl UiPlugin {
    fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
        commands.spawn(CameraUiBundle::default());

        commands
            .spawn(text_bundle(&asset_server, 10.0))
            .with(FrameTime);
        commands.spawn(text_bundle(&asset_server, 50.0)).with(Rocks);
        commands
            .spawn(text_bundle(&asset_server, 90.0))
            .with(Projectiles);
    }

    fn update_frame_time(
        diagnostics: Res<Diagnostics>,
        mut elements: Query<&mut Text, With<FrameTime>>,
    ) {
        for mut text in elements.iter_mut() {
            FrameTime::format(&diagnostics, &mut text.value);
        }
    }

    fn update_rocks(
        mut elements: Query<&mut Text, With<Rocks>>,
        rocks: Query<&Rock>,
    ) {
        for mut text in elements.iter_mut() {
            Rocks::format(rocks.iter().count(), &mut text.value);
        }
    }

    fn update_projectiles(
        mut elements: Query<&mut Text, With<Projectiles>>,
        projectiles: Query<&Projectile>,
    ) {
        for mut text in elements.iter_mut() {
            Projectiles::format(projectiles.iter().count(), &mut text.value);
        }
    }
}
