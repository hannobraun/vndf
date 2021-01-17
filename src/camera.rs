use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::world::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create.system()).add_system(update.system());
    }
}

pub struct Focus {
    camera: Entity,
}

impl Focus {
    pub fn camera(&self) -> Entity {
        self.camera
    }
}

fn create(
    commands: &mut Commands,
    players: Query<Entity, (With<Player>, Without<Focus>)>,
) {
    for player in players.iter() {
        let camera = commands
            .spawn(Camera2dBundle::default())
            .current_entity()
            .unwrap();

        commands.insert_one(player, Focus { camera });
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

        let mut camera = transforms.get_mut(focus.camera).unwrap().0;
        let position = body.position().translation.vector;
        *camera =
            Transform::from_translation(Vec3::new(position.x, position.y, 1.0));
    }
}
