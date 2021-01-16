use std::ops::Deref;

use bevy::{
    input::{mouse::MouseWheel, system::exit_on_esc_system},
    prelude::*,
};
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{
    camera,
    input::MousePosition,
    world::{player::Player, ship::Ship},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(MousePosition::none())
            .add_system(exit_on_esc_system.system())
            .add_system(Self::handle_cursor_movement.system())
            .add_system(Self::handle_direction_setting.system())
            .add_system(Self::handle_targeting.system())
            .add_system(Self::handle_thrust_setting_change.system());
    }
}

impl InputPlugin {
    fn handle_cursor_movement(
        mut mouse_position: ResMut<Option<MousePosition>>,
        mut events: ResMut<Events<CursorMoved>>,
    ) {
        for event in events.drain() {
            *mouse_position = Some(MousePosition::from_event(event));
        }
    }

    fn handle_direction_setting(
        mouse_position: Res<Option<MousePosition>>,
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
        for (mut ship, focus, body) in ships.iter_mut() {
            if let Some(mouse_position) = mouse_position.deref() {
                let window = windows
                    .get(mouse_position.window_id())
                    .expect("Could not find window");
                let camera = transforms.get(focus.camera()).unwrap();

                let mouse_position_world =
                    mouse_position.world_position(window, camera);

                if input.pressed(MouseButton::Left) {
                    let body = bodies.get(body.handle()).unwrap();
                    ship.update_direction_setting(body, mouse_position_world);
                }
            }
        }
    }

    fn handle_targeting(
        mouse_position: Res<Option<MousePosition>>,
        input: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        mut ships: Query<&camera::Focus>,
        transforms: Query<&Transform>,
    ) {
        for focus in ships.iter_mut() {
            if let Some(mouse_position) = mouse_position.deref() {
                let window = windows
                    .get(mouse_position.window_id())
                    .expect("Could not find window");
                let camera = transforms.get(focus.camera()).unwrap();

                let mouse_position_world =
                    mouse_position.world_position(window, camera);

                if input.pressed(MouseButton::Right) {
                    // TASK: Set target to mouse position.
                    println!("Right mouse click at {:?}", mouse_position_world);
                }
            }
        }
    }

    fn handle_thrust_setting_change(
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
