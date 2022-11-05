use hell_common::transform::Transform;
use hell_error::HellResult;
use hell_input::{KeyCode, InputManager};
use hell_physics::collision::AABB2D;
use hell_physics::systems::GravitySystem;
use hell_renderer::render_data::SceneData;
use hell_renderer::vulkan::RenderData;
use hell_resources::ResourceManager;

use crate::systems::{MovementSystem, MovementData, EnemySpawnSystem, EnemyKillSystem, EneymCollisionSystem, EnvironmentCollisionSystem};



pub struct NocoruScene {
    pub scene_data: SceneData,
    pub render_data: RenderData,
    pub movement_data: Vec<MovementData>,
    pub colliders: Vec<AABB2D>,
    pub is_alive: Vec<bool>,

    gravity_system: GravitySystem,
    movement_system: MovementSystem,
    environment_collision_system: EnvironmentCollisionSystem,
    enemy_collision_system: EneymCollisionSystem,
    enemy_spawn_system: EnemySpawnSystem,
    enemy_kill_system: EnemyKillSystem,

    scrolled_distance: f32,
}

impl NocoruScene {
    pub const PLAYER_IDX: usize = 0;
    pub const ENEMY_POOL_SIZE: usize = 10;
    pub const ENTITY_COUNT: usize = Self::ENEMY_POOL_SIZE + 1;

    pub const PLAYER_MAT: &'static str = "assets/chars_v1/player_mat.yaml";
    pub const ENEMY_T1_MAT: &'static str = "assets/chars_v1/enemy_t1_mat.yaml";
    pub const ENEMY_T2_MAT: &'static str = "assets/chars_v1/enemy_t2_mat.yaml";

    pub const ENEMY_SPAWN_POS: glam::Vec3 = glam::Vec3::new(5.0, 0.0, 0.0);
    pub const ENEMY_RESET_POS: glam::Vec3 = glam::Vec3::new(5.0, -3.0, 0.0);
    pub const ENEMY_KILL_POS_X: f32 = -5.0;
    pub const WORLD_SCROLL_SPEED: f32 = 2.0;
}

impl NocoruScene {
    pub fn new() -> Self {

        let scene_data = SceneData::default();
        let render_data = RenderData::default();
        let movement_data = vec![MovementData::default(); Self::ENTITY_COUNT];
        let colliders = vec![AABB2D::default(); Self::ENTITY_COUNT];
        let is_alive = vec![false; Self::ENTITY_COUNT];

        let gravity_system = GravitySystem::default();
        let movement_system = MovementSystem::default();
        let environment_collision_system = EnvironmentCollisionSystem::default();
        let enemy_collision_system = EneymCollisionSystem::default();

        let enemy_spawn_system = EnemySpawnSystem::new(Self::ENEMY_SPAWN_POS, glam::vec2(-Self::WORLD_SCROLL_SPEED, 0.0));
        let enemy_kill_system = EnemyKillSystem::new(Self::ENEMY_RESET_POS, Self::ENEMY_KILL_POS_X);


        Self {
            scene_data,
            render_data,
            movement_data,
            colliders,
            is_alive,

            gravity_system,
            movement_system,
            environment_collision_system,
            enemy_collision_system,
            enemy_spawn_system,
            enemy_kill_system,

            scrolled_distance: 0.0,
        }
    }

    pub fn reset_scene(&mut self) {
        println!("reset scene");
    }


    pub fn load_scene(&mut self, resource_manager: &mut ResourceManager) -> HellResult<()> {
        // setup player
        // ------------
        let player_mat = resource_manager.load_material(Self::PLAYER_MAT)?;
        self.render_data.add_data(0, player_mat, Transform::default());

        // setup enemies
        // -------------
        let enemy_t1_mat_idx = resource_manager.load_material(Self::ENEMY_T1_MAT)?;
        let _enemy_t2_mat_idx = resource_manager.load_material(Self::ENEMY_T2_MAT)?;
        for _ in 0..Self::ENEMY_POOL_SIZE {
            self.render_data.add_data(0, enemy_t1_mat_idx, Transform::default());
        }

        self.enemy_spawn_system.prepare(&mut self.render_data.transforms[1..Self::ENEMY_POOL_SIZE+1], &mut self.movement_data);

        Ok(())
    }

    pub fn update_scene(&mut self, delta_time: f32, input: &InputManager) -> HellResult<()> {
        let render_data = &mut self.render_data;

        // handle user input
        // -----------------
        let player_trans = &mut render_data.transforms[Self::PLAYER_IDX];
        if input.key_state(KeyCode::Space).is_down() {
            let mut player_offset = 0_f32;
            player_offset -= 20_f32 * delta_time;
            player_trans.translate_y(player_offset);
        }

        self.enemy_kill_system.execute(
            &mut render_data.transforms[1..Self::ENEMY_POOL_SIZE+1],
            &mut self.movement_data[1..Self::ENEMY_POOL_SIZE+1],
            &mut self.is_alive[1..Self::ENEMY_POOL_SIZE+1]
        );

        if self.scrolled_distance > 8.0 {
            self.scrolled_distance = 0.0;
            let _spawned_enemy_idx = self.enemy_spawn_system.execute(
                &mut render_data.transforms[1..Self::ENEMY_POOL_SIZE+1],
                &mut self.movement_data[1..Self::ENEMY_POOL_SIZE+1],
                &mut self.is_alive[1..Self::ENEMY_POOL_SIZE+1]
            );
        }

        let player_trans = &mut render_data.transforms[0..1];
        self.gravity_system.execute(player_trans, delta_time);
        self.movement_system.execute(delta_time, &mut render_data.transforms, &self.movement_data)?;
        self.environment_collision_system.execute(&mut render_data.transforms);

        let did_collide = self.enemy_collision_system.execute(
            &self.colliders[0],
            &self.colliders[1..],
            &render_data.transforms[0],
            &render_data.transforms[1..]
        );

        if did_collide {
            self.reset_scene();
        }

        self.scrolled_distance += Self::WORLD_SCROLL_SPEED * delta_time;

        Ok(())
    }
}
