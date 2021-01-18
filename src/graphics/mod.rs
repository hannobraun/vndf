pub mod course;
pub mod nav_marker;
pub mod rock;
pub mod ship;

use bevy::prelude::*;

pub const COLOR_PLAYER: Color = Color::rgb_linear(0.0, 0.0, 1.0);

pub const LAYER_WORLD: f32 = 0.0;
pub const LAYER_MARKER: f32 = 0.5;
pub const LAYER_UI: f32 = 1.0;

// TASK: Display target, if selected.
