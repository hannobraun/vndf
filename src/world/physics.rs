use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::rapier::geometry::ColliderHandle;

pub struct Physics;

pub struct ColliderMap(HashMap<ColliderHandle, Entity>);

impl ColliderMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: ColliderHandle, value: Entity) {
        self.0.insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
