mod course;
mod nav_marker;
mod projectile;
mod rock;
mod ship;
mod target;

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TASK: Draw background grid. Currently (Bevy 0.4) this is not
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
            .add_plugin(projectile::ProjectilePlugin)
            .add_plugin(rock::RockPlugin)
            .add_plugin(ship::ShipPlugin)
            .add_plugin(target::TargetPlugin);
    }
}
