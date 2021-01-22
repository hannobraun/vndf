use bevy::prelude::*;

#[derive(Debug)]
pub struct TargetGraphics {
    entity: Entity,
}

impl TargetGraphics {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
