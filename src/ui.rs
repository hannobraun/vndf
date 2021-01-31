use std::{fmt::Write as _, usize};

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FrameTime;

impl FrameTime {
    pub fn format(diagnostics: &Diagnostics, s: &mut String) {
        if let Some(frame_time) =
            diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_s) = frame_time.average() {
                s.clear();
                let _ =
                    write!(s, "Frame Time: {:.0} ms", frame_time_s * 1000.0);
            }
        }
    }
}

pub struct Rocks;

impl Rocks {
    pub fn format(num_rocks: usize, s: &mut String) {
        s.clear();
        let _ = write!(s, "Rocks: {}", num_rocks);
    }
}
pub struct Projectiles;

impl Projectiles {
    pub fn format(num_projectiles: usize, s: &mut String) {
        s.clear();
        let _ = write!(s, "Projectiles: {}", num_projectiles);
    }
}

pub fn text_bundle(asset_server: &AssetServer, top: f32) -> TextBundle {
    TextBundle {
        text: Text {
            font: asset_server.load("fonts/Tuffy_Bold.ttf"),
            style: TextStyle {
                font_size: 32.0,
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(top),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
