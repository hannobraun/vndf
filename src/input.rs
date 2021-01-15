use bevy::{prelude::*, window::WindowId};

pub struct MousePosition {
    position: Vec2,
    window_id: WindowId,
}

impl MousePosition {
    pub fn none() -> Option<Self> {
        None
    }

    pub fn from_event(event: CursorMoved) -> Self {
        Self {
            position: event.position,
            window_id: event.id,
        }
    }

    pub fn world_position(&self, window: &Window, camera: &Transform) -> Vec2 {
        let window_size =
            Vec2::new(window.width() as f32, window.height() as f32) / 2.0;
        let position = self.position - window_size;
        let position =
            camera.compute_matrix() * position.extend(0.0).extend(1.0);

        Vec2::new(position.x, position.y)
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }
}
