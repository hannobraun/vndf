use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{camera::Focus, world::ship::Ship};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::create.system())
            .add_system(Self::update.system());
    }
}

impl CameraPlugin {
    fn create(
        commands: &mut Commands,
        ships: Query<Entity, (With<Ship>, Without<Focus>)>,
    ) {
        for ship in ships.iter() {
            let camera = commands
                .spawn(Camera2dBundle::default())
                .current_entity()
                .unwrap();

            commands.insert_one(ship, Focus::new(camera));
        }
    }

    fn update(
        bodies: Res<RigidBodySet>,
        foci: Query<(&Focus, &RigidBodyHandleComponent)>,
        mut transforms: Query<(&mut Transform,)>,
    ) {
        for (focus, body) in foci.iter() {
            let body = bodies
                .get(body.handle())
                .expect("Could not find body for ship");
            let mut camera = transforms.get_mut(focus.camera()).unwrap().0;

            Focus::update_camera(body, &mut camera);
        }
    }
}
