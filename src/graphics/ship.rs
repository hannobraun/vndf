use bevy::prelude::*;
use bevy_rapier2d::{
    na, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::world::ship::{Ship, SHIP_SIZE};

use super::{COLOR_PLAYER, LAYER_MARKER, LAYER_WORLD};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(setup.system())
            .add_system(set_layer.system())
            .add_system(update_heading.system());
    }
}

pub struct Heading {
    entity: Entity,
}

fn setup(
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
                    sprite: Sprite::new(SHIP_SIZE.into()),
                    ..Default::default()
                },
            )
            .insert_one(ship, Heading { entity: heading });
    }
}

fn set_layer(mut ships: Query<&mut Transform, With<Ship>>) {
    for mut transform in ships.iter_mut() {
        transform.translation.z = LAYER_WORLD;
    }
}

fn update_heading(
    bodies: Res<RigidBodySet>,
    ships: Query<(&RigidBodyHandleComponent, &Heading), With<Ship>>,
    mut headings: Query<&mut Transform>,
) {
    for (body, heading) in ships.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let mut heading = headings.get_mut(heading.entity).unwrap();

        let offset = body.position().rotation * na::Vector2::new(200.0, 0.0);
        let position = body.position().translation.vector + offset;
        *heading = Transform::from_translation(Vec3::new(
            position.x,
            position.y,
            LAYER_MARKER,
        ));
    }
}
