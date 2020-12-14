use bevy::prelude::*;
use bevy_rapier2d::{
    na::{self, Vector2},
    physics::RigidBodyHandleComponent,
    rapier::dynamics::RigidBodySet,
};
use na::Rotation2;

use crate::world::ship::Ship;

use super::LAYER_UI;

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create.system()).add_system(update.system());
    }
}

const LENGTH: f32 = 1000.0;

struct Course {
    entity: Entity,
}

fn create(
    mut commands: Commands,
    ships: Query<Without<Course, (Entity, &Ship)>>,
) {
    for (ship, _) in ships.iter() {
        let course = commands
            .spawn(SpriteComponents {
                sprite: Sprite::new(Vec2::new(LENGTH, 1.0)),
                ..Default::default()
            })
            .current_entity()
            .unwrap();

        commands.insert_one(ship, Course { entity: course });
    }
}

fn update(
    bodies: Res<RigidBodySet>,
    ships: Query<(&Ship, &RigidBodyHandleComponent, &Course)>,
    mut courses: Query<&mut Transform>,
) {
    for (_, body, course) in ships.iter() {
        let mut transform = courses.get_mut(course.entity).unwrap();
        let body = bodies.get(body.handle()).unwrap();

        let translation = body.position().translation;
        let rotation =
            Rotation2::rotation_between(&Vector2::new(1.0, 0.0), body.linvel())
                .angle();

        let translation = Transform::from_translation(Vec3::new(
            translation.x,
            translation.y,
            LAYER_UI,
        ));
        let offset =
            Transform::from_translation(Vec3::new(LENGTH / 2.0, 0.0, 0.0));
        let rotation =
            Transform::from_rotation(Quat::from_rotation_z(rotation));

        *transform = translation.mul_transform(rotation).mul_transform(offset);
    }
}
