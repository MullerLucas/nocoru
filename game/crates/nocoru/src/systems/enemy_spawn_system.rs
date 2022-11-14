use glam::{Vec3, Vec2};
use hell_common::transform::Transform;
use super::MovementData;




pub struct EnemySpawnSystem {
    initial_velocity: Vec2,
}

impl EnemySpawnSystem {
    pub fn new(initial_velocity: Vec2) -> Self {
        Self {
            initial_velocity,
        }
    }

    pub fn prepare(&self, spawn_pos: &Vec3, transforms: &mut [Transform], movement: &mut [MovementData]) {
        for t in transforms {
            t.translation = *spawn_pos;
        }

        for m in movement {
            m.velocity = glam::Vec2::ZERO;
        }
    }

    pub fn execute(
        &self,
        spawn_pos: Vec3,
        transforms: &mut [Transform],
        movement: &mut [MovementData],
        is_alive: &mut [bool],
    ) -> Option<usize> {
        let spawn_idx = is_alive.iter().position(|alive| !*alive)?;

        transforms[spawn_idx].translation = spawn_pos;
        movement[spawn_idx].velocity = self.initial_velocity;
        is_alive[spawn_idx] = true;

        println!("spawning enemy '{}'", spawn_idx);

        Some(spawn_idx)
    }
}
