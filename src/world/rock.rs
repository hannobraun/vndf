use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::na::Vector2;
use decorum::R32;
use rand::{thread_rng, Rng as _};

pub struct Rock {
    size: f32,
}

impl Rock {
    pub fn new(size: f32) -> Self {
        Self { size }
    }

    pub fn size(&self) -> f32 {
        self.size
    }
}

pub struct RockSpawner {
    rocks: HashMap<(R32, R32), Entity>,
}

impl RockSpawner {
    const BLOCK_SIZE: f32 = 1000.0;

    pub fn new() -> Self {
        RockSpawner {
            rocks: HashMap::new(),
        }
    }

    // TASK: Improve rock generation algorithm:
    //       - Spawn at random positions, not on a grid.
    //       - Vary min and max size in a more interesting way.
    //       - Vary rock density, according to position.
    pub fn spawn(
        &mut self,
        pos: Vector2<f32>,
        mut spawn: impl FnMut(Vector2<f32>, f32) -> Entity,
    ) {
        // Snap center to a grid defined by the block size.
        let center = pos
            .map(|v| ((v / Self::BLOCK_SIZE).floor() + 0.5) * Self::BLOCK_SIZE);

        let offsets = [
            Vector2::new(-Self::BLOCK_SIZE, -Self::BLOCK_SIZE),
            Vector2::new(-Self::BLOCK_SIZE, 0.0),
            Vector2::new(-Self::BLOCK_SIZE, Self::BLOCK_SIZE),
            Vector2::new(0.0, -Self::BLOCK_SIZE),
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, Self::BLOCK_SIZE),
            Vector2::new(Self::BLOCK_SIZE, -Self::BLOCK_SIZE),
            Vector2::new(Self::BLOCK_SIZE, 0.0),
            Vector2::new(Self::BLOCK_SIZE, Self::BLOCK_SIZE),
        ];

        for &offset in &offsets {
            self.spawn_block(center + offset, &mut spawn);
        }
    }

    pub fn clean_up(
        &mut self,
        ship_position: Vector2<f32>,
        rock_position: &impl Fn(Entity) -> Option<Vector2<f32>>,
        remove: &mut impl FnMut(Entity),
    ) {
        self.rocks
            .retain(|_, &mut entity| match rock_position(entity) {
                Some(rock_position) => {
                    let distance = (ship_position - rock_position).magnitude();
                    let remove_rock = distance >= Self::BLOCK_SIZE * 3.0;

                    if remove_rock {
                        remove(entity)
                    }

                    !remove_rock
                }
                None => true,
            });
    }

    fn spawn_block(
        &mut self,
        center: Vector2<f32>,
        spawn: &mut impl FnMut(Vector2<f32>, f32) -> Entity,
    ) {
        let area = Rect {
            left: center.x - Self::BLOCK_SIZE / 2.0,
            right: center.x + Self::BLOCK_SIZE / 2.0,
            top: center.y - Self::BLOCK_SIZE / 2.0,
            bottom: center.y + Self::BLOCK_SIZE / 2.0,
        };

        let mut rng = thread_rng();

        let mut position = Vector2::new(area.left, area.top);

        loop {
            let parameters = self.parameters(position);

            if parameters.density > 0.0 {
                let position_real =
                    (R32::from_inner(position.x), R32::from_inner(position.y));

                if !self.rocks.contains_key(&position_real) {
                    let size = parameters.min_size
                        + (parameters.max_size - parameters.min_size)
                            * rng.gen::<f32>();
                    let entity = spawn(position, size);
                    self.rocks.insert(position_real, entity);

                    debug!(
                        "Spawning rock \
                        (center: ({}, {}); pos: ({}, {}, total: {})",
                        center.x,
                        center.y,
                        position_real.0,
                        position_real.1,
                        self.rocks.len(),
                    );
                }
            }

            position.x += 500.0;
            if position.x > area.right {
                position.y += 500.0;
                position.x = area.left;
            }
            if position.y > area.bottom {
                break;
            }
        }
    }

    fn parameters(&self, position: Vector2<f32>) -> SpawnParameters {
        let density = if position.y >= 0.0 { 1.0 } else { 0.0 };

        const MIN_MIN_SIZE: f32 = 10.0;
        const MAX_MIN_SIZE: f32 = 250.0;

        let f = position.x % 5000.0 / 5000.0;

        let min_size = MIN_MIN_SIZE + (MAX_MIN_SIZE - MIN_MIN_SIZE) * f;
        let max_size = min_size * 2.0;

        SpawnParameters {
            density,
            min_size,
            max_size,
        }
    }
}

struct SpawnParameters {
    density: f32,
    min_size: f32,
    max_size: f32,
}
