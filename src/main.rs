mod graphics;
mod input;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_rapier2d::{
    na,
    na::UnitComplex,
    physics::{
        RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent,
    },
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
            .add_system(update_heading.system())
            .add_system(update_nav_marker.system());
    }
}

const COLOR_PLAYER: Color = Color::rgb_linear(0.0, 0.0, 1.0);
const COLOR_ENEMY: Color = Color::rgb_linear(1.0, 0.0, 0.0);

const LAYER_MARKER: f32 = 0.5;
const LAYER_UI: f32 = 1.0;

// TASK: Split `Ship` into two components: One with data relevant to gameplay,
//       one with data relevant to graphics.
// TASK: Add system that applies thrust to body.
struct Ship {
    heading: Entity,
    thrust: f32,
}

pub struct Player {
    camera: Entity,
    nav_marker: NavMarker,
}

struct NavMarker {
    entity: Entity,
    direction: Vec2,
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

    let nav_marker = commands
        .spawn((Transform::default(),))
        .with_bundle(SpriteComponents {
            material: materials.add(Color::rgb_linear(1.0, 1.0, 1.0).into()),
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
        nav_marker: NavMarker {
            entity: nav_marker,
            direction: Vec2::unit_x(),
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
            heading,
            thrust: 0.0,
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
    mut bodies: ResMut<RigidBodySet>,
    mut players: Query<(&Player, &RigidBodyHandleComponent)>,
) {
    for (player, body) in players.iter_mut() {
        let body = bodies.get_mut(body.handle()).unwrap();

        let nav_marker_angle =
            Vec2::unit_x().angle_between(player.nav_marker.direction);

        body.set_position(
            Isometry::from_parts(
                body.position().translation,
                UnitComplex::from_angle(nav_marker_angle),
            ),
            true,
        );
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

        let offset = body.position().rotation * na::Vector2::new(200.0, 0.0);
        let position = body.position().translation.vector + offset;
        *heading = Transform::from_translation(Vec3::new(
            position.x,
            position.y,
            LAYER_MARKER,
        ));
    }
}

// TASK: Move to dedicated module, `graphics::nav_marker`.
// TASK: Split into two system, one that updates position, another that updates
//       size.
fn update_nav_marker(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &Ship, &RigidBodyHandleComponent)>,
    mut nav_markers: Query<(&mut Transform, &mut Sprite)>,
) {
    for (player, ship, body) in players.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let (mut transform, mut sprite) =
            nav_markers.get_mut(player.nav_marker.entity).unwrap();

        let dir = player.nav_marker.direction.normalize();

        let position = body.position().translation.vector
            + na::Vector2::new(dir.x(), dir.y()) * 250.0;
        *transform = Transform::from_translation(Vec3::new(
            position.x, position.y, LAYER_UI,
        ));

        let min_size = 5.0;
        let max_size = 25.0;
        let size = min_size + (max_size - min_size) * ship.thrust;
        *sprite = Sprite::new(Vec2::new(size, size));
    }
}
