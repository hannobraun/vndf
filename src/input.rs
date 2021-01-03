use std::ops::Deref;

use bevy::{
    input::{mouse::MouseWheel, system::exit_on_esc_system},
    prelude::*,
    window::WindowId,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent,
    rapier::dynamics::RigidBodySet,
};

use crate::{
    camera,
    world::{player::Player, ship::Ship},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(exit_on_esc_system.system())
            .add_system(Self::handle_mouse_click.system())
            .add_system(Self::handle_mouse_wheel.system());
    }
}

impl InputPlugin {
    fn handle_mouse_click(
        mut state: Local<Option<MousePosition>>,
        mut events: ResMut<Events<CursorMoved>>,
        input: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        bodies: Res<RigidBodySet>,
        mut ships: Query<(
            &mut Ship,
            &camera::Focus,
            &RigidBodyHandleComponent,
        )>,
        transforms: Query<&Transform>,
    ) {
        for event in events.drain() {
            *state = Some(MousePosition {
                position: event.position,
                window_id: event.id,
            });
        }

        if input.pressed(MouseButton::Left) {
            if let Some(state) = state.deref() {
                for (mut ship, focus, body) in ships.iter_mut() {
                    let window = windows
                        .get(state.window_id)
                        .expect("Could not find window");
                    let size = Vec2::new(
                        window.width() as f32,
                        window.height() as f32,
                    ) / 2.0;

                    let camera = transforms.get(focus.camera()).unwrap();

                    let body = bodies.get(body.handle()).unwrap();

                    let position = state.position - size;
                    let position = camera.compute_matrix()
                        * position.extend(0.0).extend(1.0);

                    let direction = Vector2::new(position.x, position.y)
                        - body.position().translation.vector;
                    ship.direction_setting =
                        Vec2::new(direction.x, direction.y);
                }
            }
        }
    }

    fn handle_mouse_wheel(
        mut events: ResMut<Events<MouseWheel>>,
        mut ships: Query<&mut Ship, With<Player>>,
    ) {
        for event in events.drain() {
            for mut ship in ships.iter_mut() {
                ship.change_thrust_setting(event.y / 10.0);
            }
        }
    }
}

struct MousePosition {
    position: Vec2,
    window_id: WindowId,
}
