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
        app.add_system(Self::create.system())
            .add_system(Self::update.system());
    }
}

impl CoursePlugin {
    fn create(
        commands: &mut Commands,
        ships: Query<(Entity, &Ship), Without<Course>>,
    ) {
        for (ship, _) in ships.iter() {
            let course = commands
                .spawn(SpriteBundle::default())
                .current_entity()
                .unwrap();

            commands.insert_one(ship, Course { entity: course });
        }
    }

    fn update(
        bodies: Res<RigidBodySet>,
        ships: Query<(&Ship, &RigidBodyHandleComponent, &Course)>,
        mut courses: Query<(&mut Sprite, &mut Transform)>,
    ) {
        for (_, body, course) in ships.iter() {
            let (mut sprite, mut transform) =
                courses.get_mut(course.entity).unwrap();
            let body = bodies.get(body.handle()).unwrap();

            let translation = body.position().translation;
            let rotation = Rotation2::rotation_between(
                &Vector2::new(1.0, 0.0),
                body.linvel(),
            )
            .angle();

            let speed = body.linvel().magnitude();
            let length = speed * 30.0; // show course for next 30 seconds

            let translation = Transform::from_translation(Vec3::new(
                translation.x,
                translation.y,
                LAYER_UI,
            ));
            let offset =
                Transform::from_translation(Vec3::new(length / 2.0, 0.0, 0.0));
            let rotation =
                Transform::from_rotation(Quat::from_rotation_z(rotation));

            *sprite = Sprite::new(Vec2::new(length, 1.0));
            *transform =
                translation.mul_transform(rotation).mul_transform(offset);
        }
    }
}

struct Course {
    entity: Entity,
}
