use std::ops::Deref;

use bevy::{input, prelude::*};
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
        app.add_plugin(RapierPhysicsPlugin)
            .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.15)))
            .add_startup_system(setup.system())
            .add_system(update_camera.system())
            .add_system(handle_mouse_click.system());
    }
}

struct Ship(&'static str);
struct Player {
    camera: Entity,
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

    spawn_ship(
        "player",
        Vec2::new(0.0, 0.0),
        Color::rgb(1.0, 1.0, 0.0),
        Some(camera),
        &mut commands,
        &mut materials,
    );
    spawn_ship(
        "enemy",
        Vec2::new(0.0, 200.0),
        Color::rgb(1.0, 0.0, 0.0),
        None,
        &mut commands,
        &mut materials,
    );
}

fn spawn_ship(
    name: &'static str,
    position: Vec2,
    color: Color,
    is_player: Option<Entity>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let size = Vec2::new(150.0, 50.0);

    let commands = commands
        .spawn((Ship(name),))
        .with(
            RigidBodyBuilder::new_dynamic()
                .translation(position.x(), position.y())
                .linvel(10.0, 10.0)
                .angvel(0.1),
        )
        .with(ColliderBuilder::cuboid(size.x(), size.y()))
        .with_bundle(SpriteComponents {
            material: materials.add(color.into()),
            sprite: Sprite::new(size),
            ..Default::default()
        });

    if let Some(camera) = is_player {
        commands.with(Player { camera });
    }
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
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0));
    }
}

pub struct MousePosition {
    position: Vec2,
}

fn handle_mouse_click(
    mut state: Local<Option<MousePosition>>,
    mut events: ResMut<Events<CursorMoved>>,
    input: Res<Input<MouseButton>>,
) {
    for event in events.drain() {
        *state = Some(MousePosition {
            position: event.position,
        });
    }

    // TASK: Point player ship towards mouse.
    if input.just_pressed(MouseButton::Left) {
        if let Some(state) = state.deref() {
            // TASK: Convert to world coordinates.
            println!("Left mouse button pressed at {:?}", state.position);
        }
    }
}
