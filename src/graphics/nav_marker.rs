use bevy::prelude::*;
use bevy_rapier2d::{
    na, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{ui::NavMarker, Player, Ship, LAYER_UI};

pub struct NavMarkerPlugin;

impl Plugin for NavMarkerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(add_components.system())
            .add_system(update_position.system())
            .add_system(update_size.system());
    }
}

struct NavMarkerGraphics;

fn add_components(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    nav_markers: Query<Without<NavMarkerGraphics, (Entity, &NavMarker)>>,
) {
    for (entity, _) in nav_markers.iter() {
        commands
            .insert(
                entity,
                SpriteComponents {
                    material: materials
                        .add(Color::rgb_linear(1.0, 1.0, 1.0).into()),
                    ..Default::default()
                },
            )
            .insert_one(entity, NavMarkerGraphics);
    }
}

fn update_position(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &RigidBodyHandleComponent)>,
    mut nav_markers: Query<(&NavMarker, &mut Transform)>,
) {
    for (player, body) in players.iter() {
        let body = bodies.get(body.handle()).unwrap();

        if let Ok((nav_marker, mut transform)) =
            nav_markers.get_mut(player.nav_marker)
        {
            let dir = nav_marker.direction.normalize();

            let position = body.position().translation.vector
                + na::Vector2::new(dir.x(), dir.y()) * 250.0;
            *transform = Transform::from_translation(Vec3::new(
                position.x, position.y, LAYER_UI,
            ));
        }
    }
}

fn update_size(
    players: Query<(&Player, &Ship)>,
    mut nav_markers: Query<&mut Sprite>,
) {
    for (player, ship) in players.iter() {
        if let Ok(mut sprite) = nav_markers.get_mut(player.nav_marker) {
            let min_size = 5.0;
            let max_size = 25.0;
            let size = min_size + (max_size - min_size) * ship.thrust;
            *sprite = Sprite::new(Vec2::new(size, size));
        }
    }
}
