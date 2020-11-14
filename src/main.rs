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
        app.add_resource(PrintTimer(Timer::from_seconds(1.0, true)))
            .add_startup_system(setup.system())
            .add_system(print_position.system());
    }
}

struct PrintTimer(Timer);

struct Ship(&'static str);

#[derive(Debug)]
struct Position(Vec2);

fn setup(mut commands: Commands) {
    commands
        .spawn((Ship("player"), Position(Vec2::new(0.0, 0.0))))
        .spawn((Ship("enemy"), Position(Vec2::new(0.0, 10.0))));
}

fn print_position(
    time: Res<Time>,
    mut timer: ResMut<PrintTimer>,
    query: Query<(&Ship, &Position)>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (ship, position) in query.iter() {
            println!("{}: {:?}", ship.0, position);
        }
    }
}
