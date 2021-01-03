mod camera;
mod graphics;
mod input;
mod plugins;
mod ui;
mod world;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use world::player::Player;

// TASK: Convert all free system functions into associated functions on their
//       plugins.
// TASK: Split responsibilities better: System functions should concern
//       themselves with the ECS, and getting data out of and into it. Game
//       logic itself should be handled by methods on the component types.
// TASK: Move plugins to `plugins` there. Leave pure game logic code in its
//       respective module.

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
        .add_plugin(plugins::world::WorldPlugin)
        .run();
}

fn window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: "Von Neumann Defense Force".to_owned(),
        ..WindowDescriptor::default()
    }
}
