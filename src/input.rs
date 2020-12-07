use std::ops::Deref;

use bevy::{input::mouse::MouseWheel, prelude::*, window::WindowId};
use bevy_rapier2d::{
    na, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{NavMarker, Player, Ship};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_mouse_click.system())
            .add_system(handle_mouse_wheel.system());
    }
}

struct MousePosition {
    position: Vec2,
    window_id: WindowId,
}

fn handle_mouse_click(
    mut state: Local<Option<MousePosition>>,
    mut events: ResMut<Events<CursorMoved>>,
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &RigidBodyHandleComponent)>,
    transforms: Query<&Transform>,
    mut nav_markers: Query<&mut NavMarker>,
) {
    for event in events.drain() {
        *state = Some(MousePosition {
            position: event.position,
            window_id: event.id,
        });
    }

    if input.just_pressed(MouseButton::Left) {
        if let Some(state) = state.deref() {
            for (player, body) in players.iter() {
                let window = windows
                    .get(state.window_id)
                    .expect("Could not find window");
                let size =
                    Vec2::new(window.width() as f32, window.height() as f32)
                        / 2.0;

                let position = state.position - size;

                let camera = transforms.get(player.camera).unwrap();
                let position =
                    camera.compute_matrix() * position.extend(0.0).extend(1.0);

                let body = bodies.get(body.handle()).unwrap();
                let direction = na::Vector2::new(position.x(), position.y())
                    - body.position().translation.vector;

                let mut nav_marker =
                    nav_markers.get_mut(player.nav_marker).unwrap();
                nav_marker.direction = Vec2::new(direction.x, direction.y);
            }
        }
    }
}

fn handle_mouse_wheel(
    mut events: ResMut<Events<MouseWheel>>,
    mut players: Query<(&Player, &mut Ship)>,
) {
    for event in events.drain() {
        for (_, mut ship) in players.iter_mut() {
            // TASK: Move this into an accessor method on `Ship`.
            ship.thrust += event.y / 10.0;
            ship.thrust = f32::min(f32::max(ship.thrust, 0.0), 1.0);
        }
    }
}
