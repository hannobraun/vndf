mod camera;
mod graphics;
mod input;
mod world;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_rapier2d::{na, physics::RapierConfiguration};
use slog::{info, o, Drain as _, Logger};

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = Logger::root(drain, o!());

    info!(log, "Initialized logging infrastructure.");

    App::build()
        .add_resource(log)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(crate::input::InputPlugin)
        .add_plugin(crate::graphics::GraphicsPlugin)
        .add_system(exit_on_esc_system.system())
        .run();
}

// TASK: Integrate into `world::WorldPlugin`.
struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(world::WorldPlugin)
            .add_startup_system(setup.system())
            .add_plugin(world::rock::RockPlugin);
    }
}

const COLOR_PLAYER: Color = Color::rgb_linear(0.0, 0.0, 1.0);

const LAYER_MARKER: f32 = 0.5;
const LAYER_UI: f32 = 1.0;

const SHIP_SIZE: [f32; 2] = [150.0, 50.0];

pub struct Ship {
    thrust_setting: f32,
}

pub struct Player {
    direction_setting: Vec2,
}

fn setup(
    mut commands: Commands,
    mut rapier: ResMut<RapierConfiguration>,
    log: Res<Logger>,
) {
    rapier.gravity = na::Vector2::zeros();

    commands.spawn((Player {
        direction_setting: Vec2::unit_x(),
    },));

    info!(log, "Set up world.");
}
