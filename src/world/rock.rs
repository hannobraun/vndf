use std::collections::HashSet;

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
    rocks: HashSet<(R32, R32)>,
}

impl RockSpawner {
    const BLOCK_SIZE: f32 = 1000.0;

    pub fn new() -> Self {
        RockSpawner {
            rocks: HashSet::new(),
        }
    }

    // TASK: Improve rock generation algorithm:
    //       - Spawn at random positions, not on a grid.
    //       - Vary min and max size, according to position.
    //       - Vary rock density, according to position.
    pub fn spawn(
        &mut self,
        pos: Vector2<f32>,
        mut spawn: impl FnMut(Vector2<f32>, f32),
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

    fn spawn_block(
        &mut self,
        center: Vector2<f32>,
        spawn: &mut impl FnMut(Vector2<f32>, f32),
    ) {
        let area = Rect {
            left: center.x - Self::BLOCK_SIZE / 2.0,
            right: center.x + Self::BLOCK_SIZE / 2.0,
            top: center.y - Self::BLOCK_SIZE / 2.0,
            bottom: center.y + Self::BLOCK_SIZE / 2.0,
        };

        let mut rng = thread_rng();

        let min_size = 50.0;
        let max_size = 300.0;

        let mut position = Vector2::new(area.left, area.top);

        loop {
            if position.y >= 0.0 {
                let pos =
                    (R32::from_inner(position.x), R32::from_inner(position.y));

                if !self.rocks.contains(&pos) {
                    let size =
                        min_size + (max_size - min_size) * rng.gen::<f32>();
                    spawn(position, size);
                    self.rocks.insert(pos);

                    debug!(
                        "Spawning rock \
                        (center: ({}, {}); pos: ({}, {}, total: {})",
                        center.x,
                        center.y,
                        pos.0,
                        pos.1,
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

    // TASK: Add method that cleans up rocks that are far away from the player.
    //       This should be possible using the following approach:
    //       - Convert `rocks` into `HashMap`.
    //       - Return `Entity` from the closure in `spawn_rocks`, store it in
    //         `rocks`.
    //       - When cleaning up, iterate over `HashMap`. Check actual position
    //         of each rock. If it's too far away, remove it from ECS and
    //         `rocks`.
    // TASK: Add function or method that returns rock density for any given
    //       position. This can later be extended to return a struct that
    //       returns other key parameters, like minimum and maximum size.
}
