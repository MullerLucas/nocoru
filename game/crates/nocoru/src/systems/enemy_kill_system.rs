use hell_common::transform::Transform;

use super::MovementData;

pub struct EnemyKillSystem {
    reset_pos: glam::Vec3,
    kill_pos_x: f32,
}

impl EnemyKillSystem {
    pub fn new(reset_pos: glam::Vec3, kill_pos_x: f32) -> Self {

        Self {
            reset_pos,
            kill_pos_x,
        }
    }

    pub fn execute(&self, transforms: &mut [Transform], movement: &mut [MovementData], is_alive: &mut [bool]) {
        for (idx, alive) in is_alive.iter_mut().enumerate() {
            if !*alive { continue; }

            let t = &mut transforms[idx];
            if t.translation.x > self.kill_pos_x { continue; }

            *alive = false;
            t.translation = self.reset_pos;
            movement[idx].velocity = glam::Vec2::ZERO;
        }
    }
}
