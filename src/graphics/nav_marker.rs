use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::RigidBodyHandleComponent,
    rapier::dynamics::{RigidBody, RigidBodySet},
};

use crate::world::{player::Player, ship::Ship};

use super::LAYER_UI;

pub struct NavMarker {
    // TASK: Make private
    pub entity: Entity,
}

impl NavMarker {
    pub fn update_position(
        &self,
        ship: &Ship,
        body: &RigidBody,
        transform: &mut Transform,
    ) {
        let dir = ship.direction_setting().normalize();

        let position = body.position().translation.vector
            + Vector2::new(dir.x, dir.y) * 250.0;
        *transform = Transform::from_translation(Vec3::new(
            position.x, position.y, LAYER_UI,
        ));
    }
}

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
        players: Query<Entity, (With<Player>, Without<NavMarker>)>,
    ) {
        for player in players.iter() {
            let nav_marker = commands
                .spawn(SpriteBundle {
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
        ships: Query<(&Ship, &RigidBodyHandleComponent, &NavMarker)>,
        mut nav_markers: Query<&mut Transform>,
    ) {
        for (ship, body, nav_marker) in ships.iter() {
            let body = bodies.get(body.handle()).unwrap();
            let mut transform = nav_markers.get_mut(nav_marker.entity).unwrap();

            nav_marker.update_position(ship, body, &mut transform);
        }
    }

    fn update_size(
        players: Query<(&Ship, &NavMarker), With<Player>>,
        mut nav_markers: Query<&mut Sprite>,
    ) {
        for (ship, nav_marker) in players.iter() {
            if let Ok(mut sprite) = nav_markers.get_mut(nav_marker.entity) {
                let min_size = 5.0;
                let max_size = 25.0;
                let size =
                    min_size + (max_size - min_size) * ship.thrust_setting();
                *sprite = Sprite::new(Vec2::new(size, size));
            }
        }
    }
}
