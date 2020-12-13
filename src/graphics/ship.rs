use bevy::prelude::*;
use bevy_rapier2d::{
    na, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{
    Enemy, Player, Ship, COLOR_ENEMY, COLOR_PLAYER, LAYER_MARKER, SHIP_SIZE,
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(setup.system())
            .add_system(update_heading.system());
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ships: Query<With<Ship, Entity>>,
    players: Query<With<Player, ()>>,
    enemies: Query<With<Enemy, ()>>,
) {
    for ship in ships.iter() {
        let is_player = players.get(ship).is_ok();
        let is_enemy = enemies.get(ship).is_ok();

        let color = match (is_player, is_enemy) {
            (true, false) => COLOR_PLAYER,
            (false, true) => COLOR_ENEMY,

            (true, true) => panic!("Ship is both player and enemy"),
            (false, false) => panic!("Ship is neither player nor enemy"),
        };

        commands.insert(
            ship,
            SpriteComponents {
                material: materials.add(color.into()),
                sprite: Sprite::new(SHIP_SIZE.into()),
                ..Default::default()
            },
        );
    }
}

fn update_heading(
    bodies: Res<RigidBodySet>,
    ships: Query<(&Ship, &RigidBodyHandleComponent)>,
    mut headings: Query<&mut Transform>,
) {
    for (ship, body) in ships.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let mut heading = headings.get_mut(ship.heading).unwrap();

        let offset = body.position().rotation * na::Vector2::new(200.0, 0.0);
        let position = body.position().translation.vector + offset;
        *heading = Transform::from_translation(Vec3::new(
            position.x,
            position.y,
            LAYER_MARKER,
        ));
    }
}
