use bevy::prelude::*;
use bevy_rapier2d::{
    na, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{Player, Ship, LAYER_UI};

struct NavMarker {
    // TASK: Make private, once this lives in `graphics::nav_marker`.
    pub entity: Entity,
}

pub struct NavMarkerPlugin;

impl Plugin for NavMarkerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(add_components.system())
            .add_system(update_position.system())
            .add_system(update_size.system());
    }
}

fn add_components(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<With<Player, Without<NavMarker, Entity>>>,
) {
    for player in players.iter() {
        let nav_marker = commands
            .spawn(SpriteComponents {
                material: materials
                    .add(Color::rgb_linear(1.0, 1.0, 1.0).into()),
                ..Default::default()
            })
            .current_entity()
            .unwrap();

        commands.insert_one(player, NavMarker { entity: nav_marker });
    }
}

fn update_position(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &RigidBodyHandleComponent, &NavMarker)>,
    mut nav_markers: Query<&mut Transform>,
) {
    for (player, body, nav_marker) in players.iter() {
        let body = bodies.get(body.handle()).unwrap();

        if let Ok(mut transform) = nav_markers.get_mut(nav_marker.entity) {
            let dir = player.direction_setting.normalize();

            let position = body.position().translation.vector
                + na::Vector2::new(dir.x(), dir.y()) * 250.0;
            *transform = Transform::from_translation(Vec3::new(
                position.x, position.y, LAYER_UI,
            ));
        }
    }
}

fn update_size(
    // TASK: Simplify query.
    players: Query<(&Player, &Ship, &NavMarker)>,
    mut nav_markers: Query<&mut Sprite>,
) {
    for (_, ship, nav_marker) in players.iter() {
        if let Ok(mut sprite) = nav_markers.get_mut(nav_marker.entity) {
            let min_size = 5.0;
            let max_size = 25.0;
            let size = min_size + (max_size - min_size) * ship.thrust_setting;
            *sprite = Sprite::new(Vec2::new(size, size));
        }
    }
}
