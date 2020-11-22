use std::ops::Deref;

use bevy::{input, prelude::*, window::WindowId};
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
    // TASK: Set window title to "Von Neumann Defense Force"
    // TASK: Draw background grid. Currently (Bevy 0.3) this is not straight-
    //       forward. Bevy itself doesn't have support for easy 2D drawing.
    //       There's `bevy_prototype_lyon`, but that isn't quite what I'd like
    //       either.
    //       The best idea I could come up with (using `bevy_prototype_lyon`) is
    //       to not draw the grid as several lines that leave the screen, but
    //       use a single polyline instead (connecting the otherwise separate
    //       lines outside of the screen). Sounds workable, but unless I'm
    //       seized by motivation, I'd rather just wait for easier drawing in
    //       Bevy.
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_system(input::system::exit_on_esc_system.system())
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TASK: Add system that rotates ship towards targets.
        app.add_plugin(RapierPhysicsPlugin)
            .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.15)))
            .add_startup_system(setup.system())
            .add_system(update_camera.system())
            .add_system(update_heading.system())
            .add_system(update_target.system())
            .add_system(handle_mouse_click.system());
    }
}

const COLOR_PLAYER: Color = Color::rgb_linear(0.0, 0.0, 1.0);
const COLOR_ENEMY: Color = Color::rgb_linear(1.0, 0.0, 0.0);

const LAYER_MARKER: f32 = 0.5;
const LAYER_UI: f32 = 1.0;

struct Ship {
    heading: Entity,
}

struct Player {
    camera: Entity,
    target: Target,
}

struct Target {
    entity: Entity,
    direction: Vec2,
}

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

    let player = spawn_ship(
        Vec2::new(0.0, 0.0),
        COLOR_PLAYER,
        &mut commands,
        &mut materials,
    );
    spawn_ship(
        Vec2::new(0.0, 200.0),
        COLOR_ENEMY,
        &mut commands,
        &mut materials,
    );

    let target = commands
        .spawn((Transform::default(),))
        .with_bundle(SpriteComponents {
            material: materials.add(Color::rgb_linear(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        })
        .current_entity()
        .unwrap();

    commands.insert_one(
        player,
        Player {
            camera,
            target: Target {
                entity: target,
                direction: Vec2::unit_x(),
            },
        },
    );
}

fn spawn_ship(
    position: Vec2,
    color: Color,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
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
        })
        .current_entity()
        .unwrap()
}

fn update_camera(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &RigidBodyHandleComponent)>,
    mut transforms: Query<(&mut Transform,)>,
) {
    for (player, body) in players.iter() {
        let body = bodies
            .get(body.handle())
            .expect("Could not find body for ship");

        let mut camera = transforms.get_mut(player.camera).unwrap().0;
        let position = body.position.translation.vector;
        *camera =
            Transform::from_translation(Vec3::new(position.x, position.y, 1.0));
    }
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

pub struct MousePosition {
    position: Vec2,
    window_id: WindowId,
}

fn handle_mouse_click(
    mut state: Local<Option<MousePosition>>,
    mut events: ResMut<Events<CursorMoved>>,
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    bodies: Res<RigidBodySet>,
    mut players: Query<(&mut Player, &RigidBodyHandleComponent)>,
    transforms: Query<(&Transform,)>,
) {
    for event in events.drain() {
        *state = Some(MousePosition {
            position: event.position,
            window_id: event.id,
        });
    }

    if input.just_pressed(MouseButton::Left) {
        if let Some(state) = state.deref() {
            for (mut player, body) in players.iter_mut() {
                let window = windows
                    .get(state.window_id)
                    .expect("Could not find window");
                let size =
                    Vec2::new(window.width() as f32, window.height() as f32)
                        / 2.0;

                let position = state.position - size;

                let camera = transforms.get(player.camera).unwrap().0;
                let position =
                    camera.compute_matrix() * position.extend(0.0).extend(1.0);

                let body = bodies.get(body.handle()).unwrap();
                let direction = na::Vector2::new(position.x(), position.y())
                    - body.position.translation.vector;

                player.target.direction = Vec2::new(direction.x, direction.y);
            }
        }
    }
}
