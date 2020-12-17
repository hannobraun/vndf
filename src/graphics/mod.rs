pub mod course;
pub mod nav_marker;
pub mod rock;
pub mod ship;

use bevy::prelude::*;

const COLOR_PLAYER: Color = Color::rgb_linear(0.0, 0.0, 1.0);

const LAYER_WORLD: f32 = 0.0;
const LAYER_MARKER: f32 = 0.5;
const LAYER_UI: f32 = 1.0;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TASK: Draw background grid. Currently (Bevy 0.3) this is not
        //       straight-forward. Bevy itself doesn't have support for easy 2D
        //       drawing. There's `bevy_prototype_lyon`, but that isn't quite
        //       what I'd like either.
        //       The best idea I could come up with (using
        //       `bevy_prototype_lyon`) is to not draw the grid as several lines
        //       that leave the screen, but use a single polyline instead
        //       (connecting the otherwise separate lines outside of the
        //       screen). Sounds workable, but unless I'm seized by motivation,
        //       I'd rather just wait for easier drawing in Bevy.
        app.add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.15)))
            .add_plugin(course::CoursePlugin)
            .add_plugin(nav_marker::NavMarkerPlugin)
            .add_plugin(rock::RockPlugin)
            .add_plugin(ship::ShipPlugin);
    }
}
