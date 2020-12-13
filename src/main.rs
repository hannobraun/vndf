mod camera;
mod graphics;
mod input;
mod world;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_rapier2d::{
    na,
    na::UnitComplex,
    physics::{RapierConfiguration, RigidBodyHandleComponent},
    rapier::math::Isometry,
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};
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
            .add_system(rotate_ship.system())
            .add_system(accelerate_ship.system())
            .add_plugin(world::rock::RockPlugin);
    }
}

const COLOR_PLAYER: Color = Color::rgb_linear(0.0, 0.0, 1.0);
const COLOR_ENEMY: Color = Color::rgb_linear(1.0, 0.0, 0.0);

const LAYER_MARKER: f32 = 0.5;
const LAYER_UI: f32 = 1.0;

const SHIP_SIZE: [f32; 2] = [150.0, 50.0];

// TASK: Split `Ship` into two components: One with data relevant to gameplay,
//       one with data relevant to graphics.
pub struct Ship {
    heading: Entity,
    thrust_setting: f32,
}

pub struct Player {
    direction_setting: Vec2,
}

pub struct Enemy;

fn setup(
    mut commands: Commands,
    mut rapier: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    log: Res<Logger>,
) {
    rapier.gravity = na::Vector2::zeros();

    spawn_ship(
        Vec2::new(0.0, 0.0),
        COLOR_PLAYER,
        &mut commands,
        &mut materials,
    )
    .with(Player {
        direction_setting: Vec2::unit_x(),
    });
    spawn_ship(
        Vec2::new(0.0, 200.0),
        COLOR_ENEMY,
        &mut commands,
        &mut materials,
    )
    .with(Enemy);

    info!(log, "Set up world.");
}

fn spawn_ship<'c>(
    position: Vec2,
    color: Color,
    commands: &'c mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> &'c mut Commands {
    // TASK: Move to `graphics`.
    let heading = commands
        .spawn((Transform::default(),))
        .with_bundle(SpriteComponents {
            material: materials.add(color.into()),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        })
        .current_entity()
        .unwrap();

    commands
        .spawn((Ship {
            heading,
            thrust_setting: 0.0,
        },))
        .with(
            RigidBodyBuilder::new_dynamic()
                .translation(position.x(), position.y())
                .linvel(10.0, 10.0)
                .angvel(0.5),
        )
        .with(ColliderBuilder::cuboid(
            SHIP_SIZE[0] / 2.0,
            SHIP_SIZE[1] / 2.0,
        ));

    commands
}

fn rotate_ship(
    mut bodies: ResMut<RigidBodySet>,
    mut players: Query<(&Player, &RigidBodyHandleComponent)>,
) {
    for (player, body) in players.iter_mut() {
        let body = bodies.get_mut(body.handle()).unwrap();

        let nav_marker_angle =
            Vec2::unit_x().angle_between(player.direction_setting);

        body.set_position(
            Isometry::from_parts(
                body.position().translation,
                UnitComplex::from_angle(nav_marker_angle),
            ),
            true,
        );
    }
}

fn accelerate_ship(
    mut bodies: ResMut<RigidBodySet>,
    ships: Query<(&Ship, &RigidBodyHandleComponent)>,
) {
    for (ship, body) in ships.iter() {
        let body = bodies.get_mut(body.handle()).unwrap();

        let direction = body.position().rotation * na::Vector2::new(1.0, 0.0);

        let thrust = 1_000_000.0 * direction;
        body.apply_force(ship.thrust_setting * thrust, true);
    }
}
