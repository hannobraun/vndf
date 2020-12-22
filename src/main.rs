mod camera;
mod graphics;
mod input;
mod ui;
mod world;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use world::player::Player;

fn main() {
    App::build()
        // Needs to be placed before the default plugins:
        // https://github.com/bevyengine/bevy/issues/278
        .add_resource(window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(world::WorldPlugin)
        .run();
}

fn window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: "Von Neumann Defense Force".to_owned(),
        ..WindowDescriptor::default()
    }
}
