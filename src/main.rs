mod graphics;
mod input;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_rapier2d::{
    na,
    physics::{
        RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent,
    },
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};

fn main() {
    App::build()
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
        // TASK: Add system that rotates ship towards targets.
        app.add_plugin(RapierPhysicsPlugin)
            .add_startup_system(setup.system())
            .add_system(update_heading.system())
            .add_system(update_target.system());
    }
}

const COLOR_PLAYER: Color = Color::rgb_linear(0.0, 0.0, 1.0);
const COLOR_ENEMY: Color = Color::rgb_linear(1.0, 0.0, 0.0);

const LAYER_MARKER: f32 = 0.5;
const LAYER_UI: f32 = 1.0;

// TASK: Add thrust setting and system that applies it to body.
struct Ship {
    // TASK: Prototype turning this into general nav marker that also visualizes
    //       thrust setting through its size.
    heading: Entity,
}

pub struct Player {
    camera: Entity,
    target: Target,
}

struct Target {
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
    mut rapier_config: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    rapier_config.gravity = na::Vector2::zeros();

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
        },
    });
    spawn_ship(
        Vec2::new(0.0, 200.0),
        COLOR_ENEMY,
        &mut commands,
        &mut materials,
    )
    .with(Enemy);
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
        .spawn((Ship { heading },))
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

fn update_heading(
    bodies: Res<RigidBodySet>,
    ships: Query<(&Ship, &RigidBodyHandleComponent)>,
    mut headings: Query<(&mut Transform,)>,
) {
    for (ship, body) in ships.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let mut heading = headings.get_mut(ship.heading).unwrap().0;

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
    mut targets: Query<(&mut Transform,)>,
) {
    for (player, body) in players.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let mut target = targets.get_mut(player.target.entity).unwrap().0;

        let dir = player.target.direction.normalize();

        let position = body.position.translation.vector
            + na::Vector2::new(dir.x(), dir.y()) * 250.0;
        *target = Transform::from_translation(Vec3::new(
            position.x, position.y, LAYER_UI,
        ));
    }
}
