mod graphics;
mod input;

use std::f32::consts::PI;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_rapier2d::{
    na,
    physics::{
        RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent,
    },
    rapier::{
        dynamics::{RigidBody, RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};
use pid::Pid;
use slog::{debug, info, o, Drain as _, Logger};

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = Logger::root(drain, o!());

    info!(log, "Initialized logging infrastructure.");

    App::build()
        .add_resource(log)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(crate::input::InputPlugin)
        .add_plugin(crate::graphics::GraphicsPlugin)
        .add_system(exit_on_esc_system.system())
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin)
            .add_startup_system(setup.system())
            .add_system(rotate_ship.system())
            .add_system(update_ships.system())
            .add_system(update_heading.system())
            .add_system(update_target.system());
    }
}

const COLOR_PLAYER: Color = Color::rgb_linear(0.0, 0.0, 1.0);
const COLOR_ENEMY: Color = Color::rgb_linear(1.0, 0.0, 0.0);

const LAYER_MARKER: f32 = 0.5;
const LAYER_UI: f32 = 1.0;

// TASK: Split `Ship` into two components: One with data relevant to gameplay,
//       one with data relevant to graphics.
// TASK: Add thrust setting and a system that applies it to body.
struct Ship {
    angular_thrust: f32,
    angular_thrust_setting: f32,

    // TASK: Prototype turning this into general nav marker that also visualizes
    //       thrust setting through its size.
    heading: Entity,
}

impl Ship {
    /// Control angular thrusters
    ///
    /// `setting` will be clamped to the range from `-1.0` to `1.0`.
    fn control_angular_thrusters(&mut self, setting: f32, log: &Logger) {
        let setting = f32::max(f32::min(setting, 1.0), -1.0);

        if setting != self.angular_thrust_setting {
            debug!(log, "Controlling angular thrusters"; "setting" => setting);
            self.angular_thrust_setting = setting;
        }
    }

    fn update(&self, body: &mut RigidBody) {
        let impulse = self.angular_thrust_setting * self.angular_thrust;
        body.apply_torque_impulse(impulse);
    }
}

pub struct Player {
    camera: Entity,
    target: Target,
}

struct Target {
    entity: Entity,
    direction: Vec2,
    control: Pid<f32>,
}

pub struct Enemy;

// TASK: Extract graphics-specific parts and move them to a new `setup` system
//       in `graphics`. This `setup` system would still create the entities and
//       the new `graphics::setup` system would add components to them, as
//       required.
fn setup(
    mut commands: Commands,
    mut rapier: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    log: Res<Logger>,
) {
    rapier.gravity = na::Vector2::zeros();

    let camera = commands
        .spawn(Camera2dComponents::default())
        .current_entity()
        .unwrap();

    let target = commands
        .spawn((Transform::default(),))
        .with_bundle(SpriteComponents {
            material: materials.add(Color::rgb_linear(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        })
        .current_entity()
        .unwrap();

    spawn_ship(
        Vec2::new(0.0, 0.0),
        COLOR_PLAYER,
        &mut commands,
        &mut materials,
    )
    .with(Player {
        camera,
        target: Target {
            entity: target,
            direction: Vec2::unit_x(),
            // TASK: Optimize PID parameters.
            control: Pid::new(
                0.25,
                0.0,
                128.0,
                f32::INFINITY,
                f32::INFINITY,
                f32::INFINITY,
                f32::INFINITY,
                0.0,
            ),
        },
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
    let size = Vec2::new(150.0, 50.0);

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
            angular_thrust: 100_000.0,
            angular_thrust_setting: 0.0,
            heading,
        },))
        .with(
            RigidBodyBuilder::new_dynamic()
                .translation(position.x(), position.y())
                .linvel(10.0, 10.0)
                .angvel(0.5),
        )
        .with(ColliderBuilder::cuboid(size.x() / 2.0, size.y() / 2.0))
        .with_bundle(SpriteComponents {
            material: materials.add(color.into()),
            sprite: Sprite::new(size),
            ..Default::default()
        });

    commands
}

fn rotate_ship(
    log: Res<Logger>,
    bodies: Res<RigidBodySet>,
    mut players: Query<(&mut Player, &mut Ship, &RigidBodyHandleComponent)>,
) {
    for (mut player, mut ship, body) in players.iter_mut() {
        let body = bodies.get(body.handle()).unwrap();

        let current = body.position.rotation * na::Vector2::new(1.0, 0.0);
        let target = player.target.direction;
        let difference = target.angle_between(Vec2::new(current.x, current.y));

        let max_vel = PI * 2.0;
        if body.angvel.abs() > max_vel {
            let setting = (max_vel - body.angvel).signum();
            ship.control_angular_thrusters(setting, &log);
            continue;
        }

        // TASK: Replace PID controller with more aggressive, model-based
        //       control scheme.
        let output =
            player.target.control.next_control_output(difference).output;

        ship.control_angular_thrusters(output, &log);
    }
}

fn update_ships(
    mut bodies: ResMut<RigidBodySet>,
    ships: Query<(&Ship, &RigidBodyHandleComponent)>,
) {
    for (ship, body) in ships.iter() {
        let mut body = bodies.get_mut(body.handle()).unwrap();

        ship.update(&mut body);
    }
}

fn update_heading(
    bodies: Res<RigidBodySet>,
    ships: Query<(&Ship, &RigidBodyHandleComponent)>,
    mut headings: Query<&mut Transform>,
) {
    for (ship, body) in ships.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let mut heading = headings.get_mut(ship.heading).unwrap();

        let offset = body.position.rotation * na::Vector2::new(200.0, 0.0);
        let position = body.position.translation.vector + offset;
        *heading = Transform::from_translation(Vec3::new(
            position.x,
            position.y,
            LAYER_MARKER,
        ));
    }
}

fn update_target(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &RigidBodyHandleComponent)>,
    mut targets: Query<&mut Transform>,
) {
    for (player, body) in players.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let mut target = targets.get_mut(player.target.entity).unwrap();

        let dir = player.target.direction.normalize();

        let position = body.position.translation.vector
            + na::Vector2::new(dir.x(), dir.y()) * 250.0;
        *target = Transform::from_translation(Vec3::new(
            position.x, position.y, LAYER_UI,
        ));
    }
}
