use hell_common::transform::Transform;
use hell_error::{HellResult, HellError, HellErrorKind, HellErrorContent};

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
