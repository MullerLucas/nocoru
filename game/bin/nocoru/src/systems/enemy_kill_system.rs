use glam::Vec3;
use hell_common::transform::Transform;

use super::MovementData;

pub struct EnemyKillSystem {
    kill_pos_x: f32,
}

impl EnemyKillSystem {
    pub fn new(kill_pos_x: f32) -> Self {

        Self {
            kill_pos_x,
        }
    }

    pub fn execute(&self, reset_pos: &Vec3, transforms: &mut [Transform], movement: &mut [MovementData], is_alive: &mut [bool]) {
        for (idx, alive) in is_alive.iter_mut().enumerate() {
            if !*alive { continue; }

            let t = &mut transforms[idx];
            if t.translation.x > self.kill_pos_x { continue; }

            *alive = false;
            t.translation = *reset_pos;
            movement[idx].velocity = glam::Vec2::ZERO;
        }
    }
}
