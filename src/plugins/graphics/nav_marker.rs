use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{graphics::nav_marker::NavMarker, world::ship::Ship};

pub struct NavMarkerPlugin;

impl Plugin for NavMarkerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::add_components.system())
            .add_system(Self::update_position.system())
            .add_system(Self::update_size.system());
    }
}

impl NavMarkerPlugin {
    fn add_components(
        commands: &mut Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        ships: Query<Entity, (With<Ship>, Without<NavMarker>)>,
    ) {
        for ship in ships.iter() {
            let nav_marker = commands
                .spawn(SpriteBundle {
                    material: materials
                        .add(Color::rgb_linear(1.0, 1.0, 1.0).into()),
                    ..Default::default()
                })
                .current_entity()
                .unwrap();

            commands.insert_one(ship, NavMarker::new(nav_marker));
        }
    }

    fn update_position(
        bodies: Res<RigidBodySet>,
        ships: Query<(&Ship, &RigidBodyHandleComponent, &NavMarker)>,
        mut nav_markers: Query<&mut Transform>,
    ) {
        for (ship, body, nav_marker) in ships.iter() {
            let body = bodies.get(body.handle()).unwrap();
            let mut transform =
                nav_markers.get_mut(nav_marker.entity()).unwrap();

            NavMarker::update_position(ship, body, &mut transform);
        }
    }

    fn update_size(
        players: Query<(&Ship, &NavMarker)>,
        mut nav_markers: Query<&mut Sprite>,
    ) {
        for (ship, nav_marker) in players.iter() {
            let mut sprite = nav_markers.get_mut(nav_marker.entity()).unwrap();
            NavMarker::update_size(ship, &mut sprite);
        }
    }
}
