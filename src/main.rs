use bevy::{input, prelude::*};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_system(input::system::exit_on_esc_system.system())
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(print_position.system());
    }
}

#[derive(Debug)]
struct Position(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn((Position(Vec2::new(0.0, 0.0)),));
}

fn print_position(position: &Position) {
    println!("{:?}", position);
}
