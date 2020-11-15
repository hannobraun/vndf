use bevy::{input, prelude::*, render::camera::Camera};
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
    // TASK: Implement background grid.
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
            .add_system(update_camera.system());
    }
}

struct Ship(&'static str);
struct Player;

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    rapier_config.gravity = na::Vector2::zeros();

    // TASK: Follow player ship with camera.
    commands.spawn(Camera2dComponents::default());

    spawn_ship(
        "player",
        Vec2::new(0.0, 0.0),
        Color::rgb(1.0, 1.0, 0.0),
        true,
        &mut commands,
        &mut materials,
    );
    spawn_ship(
        "enemy",
        Vec2::new(0.0, 200.0),
        Color::rgb(1.0, 0.0, 0.0),
        false,
        &mut commands,
        &mut materials,
    );
}

fn spawn_ship(
    name: &'static str,
    position: Vec2,
    color: Color,
    player: bool,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let size = 50.0;

    let commands = commands
        .spawn((Ship(name),))
        .with(
            RigidBodyBuilder::new_dynamic()
                .translation(position.x(), position.y())
                .linvel(10.0, 10.0)
                .angvel(0.1),
        )
        .with(ColliderBuilder::cuboid(size, size))
        .with_bundle(SpriteComponents {
            material: materials.add(color.into()),
            sprite: Sprite::new(Vec2::new(size, size)),
            ..Default::default()
        });

    if player {
        commands.with(Player);
    }
}

fn update_camera(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &RigidBodyHandleComponent)>,
    mut cameras: Query<(&Camera, &mut Transform)>,
) {
    for (_, body) in players.iter() {
        let body = bodies
            .get(body.handle())
            .expect("Could not find body for ship");

        for (_, mut transform) in cameras.iter_mut() {
            let position = body.position.translation.vector;
            *transform = Transform::from_translation(Vec3::new(
                position.x, position.y, 0.0,
            ));
        }
    }
}
