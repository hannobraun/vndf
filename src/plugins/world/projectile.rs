use bevy::prelude::*;

use crate::world::projectile::Projectile;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system());
    }
}

impl ProjectilePlugin {
    fn setup(commands: &mut Commands) {
        commands.spawn((Projectile,));
    }
}
