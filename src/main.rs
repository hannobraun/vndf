mod camera;
mod graphics;
mod input;
mod world;

use bevy::{input::system::exit_on_esc_system, prelude::*};
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
        .add_plugins(DefaultPlugins)
        .add_plugin(world::WorldPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(crate::input::InputPlugin)
        .add_plugin(crate::graphics::GraphicsPlugin)
        .add_system(exit_on_esc_system.system())
        .run();
}
