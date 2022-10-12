use hell_common::transform::Transform;
use super::MovementData;




pub struct EnemySpawnSystem {
    spawn_pos: glam::Vec3,
    initial_velocity: glam::Vec2,
}

impl EnemySpawnSystem {
    pub fn new(spawn_pos: glam::Vec3, initial_velocity: glam::Vec2) -> Self {
        Self {
            spawn_pos,
            initial_velocity,
        }
    }

    pub fn prepare(&self, transforms: &mut [Transform], movement: &mut [MovementData]) {
        for t in transforms {
            t.translation = self.spawn_pos;
        }

        for m in movement {
            m.velocity = glam::Vec2::ZERO;
        }
    }

    pub fn execute(
        &self,
        transforms: &mut [Transform],
        movement: &mut [MovementData],
        is_alive: &mut [bool],
    ) -> Option<usize> {
        let spawn_idx = is_alive.iter().position(|alive| !*alive)?;

        transforms[spawn_idx].translation = self.spawn_pos;
        movement[spawn_idx].velocity = self.initial_velocity;
        is_alive[spawn_idx] = true;

        println!("spawning enemy '{}'", spawn_idx);

        Some(spawn_idx)
    }
}
