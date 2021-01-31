use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, _app: &mut AppBuilder) {}
}

// TASK: Destroy projectiles on impact.
// TASK: Apply force to bodies hit by projectile on impact.
// TASK: Clean up projectiles that are far away from player.
