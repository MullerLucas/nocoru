use hell_common::transform::Transform;
use hell_error::HellResult;
use hell_input::{KeyCode, InputManager};
use hell_physics::systems::{GravitySystem, CollisionSystem};
use hell_renderer::render_data::SceneData;
use hell_renderer::vulkan::RenderData;
use hell_resources::ResourceManager;

use crate::systems::{MovementSystem, MovementData};



pub struct NocoruScene {
    material_paths: Vec<&'static str>,

    pub scene_data: SceneData,
    pub render_data: RenderData,
    pub movement_data: Vec<MovementData>,

    gravity_system: GravitySystem,
    movement_system: MovementSystem,
    collision_system: CollisionSystem,
}

impl NocoruScene {
    pub const PLAYER_IDX: usize = 0;
}

impl NocoruScene {
    pub fn new_scene_1() -> Self {

        let scene_data = SceneData::default();
        let render_data = RenderData::default();

        let material_paths = vec![
            "assets/player_mat.yaml",
            "assets/enemy_1_mat.yaml",
            "assets/enemy_2_mat.yaml",
        ];

        let gravity_system = GravitySystem::default();
        let movement_system = MovementSystem::default();
        let collision_system = CollisionSystem::default();

        let entity_count = 3;
        let movement_data = vec![MovementData::default(); entity_count];


        Self {
            material_paths,

            scene_data,
            render_data,
            movement_data,

            gravity_system,
            movement_system,
            collision_system,
        }
    }

    pub fn load_scene(&mut self, resource_manager: &mut ResourceManager) -> HellResult<()> {
        // load materials
        // --------------
        for mat in &self.material_paths {
            let mat_idx = resource_manager.load_material(mat)?;
            self.render_data.add_data(0, mat_idx, Transform::default());
        }

        Ok(())
    }

    pub fn update_scene_1(&mut self, delta_time: f32, input: &InputManager) -> HellResult<()> {
        let render_data = &mut self.render_data;

        // handle user input
        // -----------------
        let player_trans = &mut render_data.transforms[Self::PLAYER_IDX];
        if input.key_state(KeyCode::Space).is_down() {
            let mut player_offset = 0_f32;
            player_offset -= 20_f32 * delta_time;
            player_trans.translate_y(player_offset);
        }

        self.movement_data[1].velocity = glam::vec2(-1.0, 0.0);
        self.movement_data[2].velocity = glam::vec2(-3.0, 0.0);

        let player_trans = &mut render_data.transforms[0..1];
        self.gravity_system.execute(player_trans, delta_time);
        self.movement_system.execute(delta_time, &mut render_data.transforms, &self.movement_data)?;
        self.collision_system.execute(&mut render_data.transforms);

        Ok(())
    }
}
