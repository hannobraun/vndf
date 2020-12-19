mod camera;
mod graphics;
mod input;
mod ui;
mod world;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
};
use slog::{info, o, Drain as _, Logger};
use world::player::Player;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = Logger::root(drain, o!());

    info!(log, "Initialized logging infrastructure.");

    App::build()
        .add_resource(log)
        // Needs to be placed before the default plugins:
        // https://github.com/bevyengine/bevy/issues/278
        .add_resource(window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // TASK: Remove
        .add_plugin(PrintDiagnosticsPlugin::default())
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
