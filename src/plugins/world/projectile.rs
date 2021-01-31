use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, _app: &mut AppBuilder) {}
}

// TASK: Destroy projectiles on impact.
// TASK: On impact, apply force to body that is hit by projectile.
// TASK: Clean up projectiles that are far away from player.
