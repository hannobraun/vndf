use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{
    graphics::{
        ship::{self, Heading},
        COLOR_PLAYER,
    },
    world::ship::{Ship, SHIP_SIZE},
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::add_components.system())
            .add_system(Self::set_layer.system())
            .add_system(Self::update_heading.system());
    }
}

impl ShipPlugin {
    fn add_components(
        commands: &mut Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        ships: Query<Entity, (With<Ship>, Without<Heading>)>,
    ) {
        for ship in ships.iter() {
            let heading = commands
                .spawn(SpriteBundle {
                    material: materials.add(COLOR_PLAYER.into()),
                    sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                    ..Default::default()
                })
                .current_entity()
                .unwrap();

            commands
                .insert(
                    ship,
                    SpriteBundle {
                        material: materials.add(COLOR_PLAYER.into()),
                        sprite: Sprite::new(SHIP_SIZE),
                        ..Default::default()
                    },
                )
                .insert_one(ship, Heading::new(heading));
        }
    }

    fn set_layer(mut ships: Query<&mut Transform, With<Ship>>) {
        for mut transform in ships.iter_mut() {
            ship::set_layer(&mut transform);
        }
    }

    fn update_heading(
        bodies: Res<RigidBodySet>,
        ships: Query<(&RigidBodyHandleComponent, &Heading), With<Ship>>,
        mut headings: Query<&mut Transform>,
    ) {
        for (body, heading) in ships.iter() {
            let body = bodies.get(body.handle()).unwrap();
            let mut heading = headings.get_mut(heading.entity()).unwrap();

            Heading::update(body, &mut heading);
        }
    }
}
