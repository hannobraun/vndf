mod camera;
mod graphics;
mod input;
mod plugins;
mod ui;
mod world;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

// TASK: Move plugins to `plugins` there. Leave pure game logic code in its
//       respective module.

fn main() {
    App::build()
        // Needs to be placed before the default plugins:
        // https://github.com/bevyengine/bevy/issues/278
        .add_resource(window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(plugins::camera::CameraPlugin)
        .add_plugin(plugins::graphics::GraphicsPlugin)
        .add_plugin(plugins::input::InputPlugin)
        .add_plugin(plugins::ui::UiPlugin)
        .add_plugin(plugins::world::WorldPlugin)
        .run();
}

fn window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: "Von Neumann Defense Force".to_owned(),
        ..WindowDescriptor::default()
    }
}
