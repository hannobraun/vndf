use std::ops::Deref;

use bevy::{prelude::*, window::WindowId};
use bevy_rapier2d::{
    na, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TASK: Add system that applies mouse wheel changes to ship thrust.
        app.add_system(handle_mouse_click.system());
    }
}

use crate::Player;

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
    mut players: Query<(&mut Player, &RigidBodyHandleComponent)>,
    transforms: Query<(&Transform,)>,
) {
    for event in events.drain() {
        *state = Some(MousePosition {
            position: event.position,
            window_id: event.id,
        });
    }

    if input.just_pressed(MouseButton::Left) {
        if let Some(state) = state.deref() {
            for (mut player, body) in players.iter_mut() {
                let window = windows
                    .get(state.window_id)
                    .expect("Could not find window");
                let size =
                    Vec2::new(window.width() as f32, window.height() as f32)
                        / 2.0;

                let position = state.position - size;

                let camera = transforms.get(player.camera).unwrap().0;
                let position =
                    camera.compute_matrix() * position.extend(0.0).extend(1.0);

                let body = bodies.get(body.handle()).unwrap();
                let direction = na::Vector2::new(position.x(), position.y())
                    - body.position.translation.vector;

                player.target.direction = Vec2::new(direction.x, direction.y);
            }
        }
    }
}
