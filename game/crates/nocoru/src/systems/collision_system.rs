use hell_common::transform::Transform;
use hell_physics::collision::AABB2D;

use super::MovementData;



pub struct EnvironmentCollisionSystem {
    floor_y: f32,
    ceiling_y: f32,
}

impl EnvironmentCollisionSystem {
    pub fn new(floor_y: f32, ceiling_y: f32) -> Self {
        Self {
            floor_y,
            ceiling_y,
        }
    }

    pub fn execute(&self, transforms: &mut [Transform], movement_data: &mut [MovementData], is_grounded: &mut [bool]) {
        transforms.iter_mut()
            .zip(movement_data)
            .zip(is_grounded)
            .for_each(|((t, md), ig)| {
                // touch ground
                if t.translation.y < self.floor_y {
                    *ig = true;
                    t.translation.y = self.floor_y;
                    md.velocity.y = md.velocity.y.max(0.0);
                }

                // touch ceiling
                if t.translation.y > self.ceiling_y {
                    t.translation.y = self.ceiling_y;
                    md.velocity.y = md.velocity.y.min(0.0);
                }
            });
    }
}

impl Default for EnvironmentCollisionSystem {
    fn default() -> Self {
        Self::new(0.0, 100.0)
    }
}



// ----------------------------------------------



#[derive(Default)]
pub struct EneymCollisionSystem;

impl EneymCollisionSystem {
    pub fn execute(&self, player_collider: &AABB2D, enemy_colliders: &[AABB2D], player_transform: &Transform, enemy_transforms: &[Transform]) -> bool {
        let pc = player_collider.transform(player_transform);

        for (c, t) in enemy_colliders.iter().zip(enemy_transforms) {
            let ec = c.transform(t);

            if pc.does_overlap(&ec) {
                return true;
            }
        }

        false
    }
}
