pub mod course;
pub mod nav_marker;
pub mod rock;
pub mod ship;

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TASK: Set window title to "Von Neumann Defense Force"
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
        // TASK: Add system that sets z coordinate of ships explicitly.
        app.add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.15)))
            .add_plugin(course::CoursePlugin)
            .add_plugin(nav_marker::NavMarkerPlugin)
            .add_plugin(rock::RocksPlugin)
            .add_plugin(ship::ShipPlugin);
    }
}
