pub mod engines;
pub mod rcs;
pub mod weapons;

use bevy::prelude::*;

use self::{engines::Engines, rcs::Rcs, weapons::Weapons};

pub const SHIP_SIZE: Vec2 = Vec2 { x: 150.0, y: 50.0 };

pub struct Ship {
    engines: Engines,
    rcs: Rcs,
    weapons: Weapons,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            engines: Engines::new(),
            rcs: Rcs::new(),
            weapons: Weapons::new(),
        }
    }

    pub fn engines(&self) -> &Engines {
        &self.engines
    }

    pub fn engines_mut(&mut self) -> &mut Engines {
        &mut self.engines
    }

    pub fn rcs(&self) -> &Rcs {
        &self.rcs
    }

    pub fn rcs_mut(&mut self) -> &mut Rcs {
        &mut self.rcs
    }

    pub fn weapons_mut(&mut self) -> &mut Weapons {
        &mut self.weapons
    }
}
