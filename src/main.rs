use bevy::{input, prelude::*};
use bevy_rapier2d::{
    physics::RapierPhysicsPlugin,
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

fn main() {
    // TASK: Set window title to "Von Neumann Defense Force"
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
            .add_startup_system(setup.system());
    }
}

struct Ship(&'static str);

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    spawn_ship(
        "player",
        Vec2::new(0.0, 0.0),
        Color::rgb(1.0, 1.0, 0.0),
        &mut commands,
        &mut materials,
    );
    spawn_ship(
        "enemy",
        Vec2::new(0.0, 200.0),
        Color::rgb(1.0, 0.0, 0.0),
        &mut commands,
        &mut materials,
    );
}

fn spawn_ship(
    name: &'static str,
    position: Vec2,
    color: Color,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let size = 100.0;

    commands
        .spawn((Ship(name),))
        .with(
            // TASK: Make dynamic. Currently Rapier simulates gravity though, so
            //       that has to be removed first.
            // TASK: Set initial linear and angular velocities.
            RigidBodyBuilder::new_static()
                .translation(position.x(), position.y()),
        )
        .with(ColliderBuilder::cuboid(size, size))
        .with_bundle(SpriteComponents {
            material: materials.add(color.into()),
            sprite: Sprite::new(Vec2::new(size, size)),
            ..Default::default()
        });
}
