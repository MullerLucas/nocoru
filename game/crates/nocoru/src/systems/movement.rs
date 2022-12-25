use hell_common::transform::Transform;
use hell_error::{HellResult, HellError, HellErrorKind, HellErrorContent};
use hell_physics::PhysicsConfig;



#[derive(Clone)]
pub struct MovementData {
    pub velocity: glam::Vec2,
}

impl Default for MovementData {
    fn default() -> Self {
        Self {
            velocity: glam::Vec2::ZERO,
        }
    }
}

#[derive(Default)]
pub struct MovementSystem;

impl MovementSystem {
    pub fn execute(&self, delta_time: f32, transforms: &mut [Transform], settings: &[MovementData]) -> HellResult<()> {
        if transforms.len() != settings.len() {
            return Err(
                HellError::new(HellErrorKind::GenericError, HellErrorContent::Message("MovementSystem received invalid input data".to_string()))
            );
        }

        for (t, s) in transforms.iter_mut().zip(settings.iter()) {
            t.translate_xy(s.velocity * delta_time);
        }

        Ok(())
    }
}



// ----------------------------------------------



#[derive(Default)]
pub struct GravitySystem {
    config: PhysicsConfig
}

impl GravitySystem {
    #[allow(dead_code)]
    pub fn new(config: PhysicsConfig) -> Self {
        Self {
            config
        }
    }

    pub fn execute(&self, movement_data: &mut [MovementData], delta_time: f32) {
        let offset = self.config.g_force * delta_time;

        for md in movement_data {
            md.velocity.y += offset;
        }
    }
}



// ----------------------------------------------



#[derive(Clone)]
pub struct JumpSystem {
    jump_force: f32,
    fall_force: f32,
}

impl JumpSystem {
    pub fn new(jump_force: f32, fall_force: f32) -> Self {
        Self { jump_force, fall_force }
    }

    pub fn execute(&self, delta_time: f32, movement_data: &mut [MovementData], is_grounded: &mut [bool], wants_to_jump: &[bool]) {
        let offset = self.jump_force;
        let fall_multi = self.fall_force * delta_time;

        for (idx, md) in movement_data.iter_mut().enumerate() {
            let ig = &mut is_grounded[idx];
            let wtj = &wants_to_jump[idx];

            if *ig && *wtj {
                // jump
                *ig = false;
                md.velocity.y += offset;
            } else if !*ig && !*wtj {
                // fall
                md.velocity.y += fall_multi;
            }
        }
    }
}
