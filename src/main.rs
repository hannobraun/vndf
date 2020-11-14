use bevy::prelude::*;

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_system(hello_world.system())
        .add_system(print_position.system())
        .run();
}

fn hello_world() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Position(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn((Position(Vec2::new(0.0, 0.0)),));
}

fn print_position(position: &Position) {
    println!("{:?}", position);
}
