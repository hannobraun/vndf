use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{graphics::course::Course, world::ship::Ship};

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
        ships: Query<(&RigidBodyHandleComponent, &Course), With<Ship>>,
        mut courses: Query<(&mut Sprite, &mut Transform)>,
    ) {
        for (body, course) in ships.iter() {
            let (mut sprite, mut transform) =
                courses.get_mut(course.entity()).unwrap();
            let body = bodies.get(body.handle()).unwrap();

            Course::update(body, &mut sprite, &mut transform);
        }
    }
}
