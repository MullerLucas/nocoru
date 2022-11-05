use hell_common::transform::Transform;
use hell_physics::collision::AABB2D;



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

    pub fn execute(&self, transforms: &mut [Transform]) {
        for t in transforms  {
            t.clamp_y(self.ceiling_y, self.floor_y);
        }
    }
}


impl Default for EnvironmentCollisionSystem {
    fn default() -> Self {
        Self::new(0.0, f32::MIN)
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



// ----------------------------------------------



