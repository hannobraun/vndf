use std::{fmt::Write as _, usize};

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

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
