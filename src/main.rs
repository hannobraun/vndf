use bevy::{input, prelude::*};

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
        app.add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.15)))
            .add_resource(PrintTimer(Timer::from_seconds(1.0, true)))
            .add_startup_system(setup.system())
            .add_system(update_transform.system());
    }
}

struct PrintTimer(Timer);

struct Ship(&'static str);

#[derive(Debug)]
struct Position(Vec2);

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
    commands
        .spawn((Ship(name), Position(position)))
        .with_bundle(SpriteComponents {
            material: materials.add(color.into()),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            ..Default::default()
        });
}

fn update_transform(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        *transform = Transform::from_translation(position.0.extend(0.0));
    }
}
