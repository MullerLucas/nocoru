use hell_common::transform::Transform;
use hell_error::HellResult;
use hell_gui::text::{HellFont, TextMesh};
use hell_input::{KeyCode, InputManager};
use hell_physics::collision::AABB2D;
use hell_renderer::render_data::SceneData;
use hell_renderer::vulkan::RenderData;
use hell_resources::ResourceManager;
use hell_resources::fonts::FntFile;

use crate::systems::{MovementSystem, MovementData, EnemySpawnSystem, EnemyKillSystem, EneymCollisionSystem, EnvironmentCollisionSystem, JumpSystem, GravitySystem};



pub struct NocoruScene {
    pub scene_data: SceneData,
    render_data: RenderData,
    pub movement_data: Vec<MovementData>,
    pub colliders: Vec<AABB2D>,
    pub is_alive: Vec<bool>,
    is_grounded: Vec<bool>,

    gravity_system: GravitySystem,
    movement_system: MovementSystem,
    jump_system: JumpSystem,
    environment_collision_system: EnvironmentCollisionSystem,
    enemy_collision_system: EneymCollisionSystem,
    enemy_spawn_system: EnemySpawnSystem,
    enemy_kill_system: EnemyKillSystem,

    ground_distance: f32,
    enemy_distance: f32,

    score_txt: TextMesh,
}

impl NocoruScene {

    pub const FLOOR_Y: f32 = -1.0;
    pub const CEILING_Y: f32 = 10.0;

    pub const GROUND_SIZE: f32 = 1.0;

    pub const GROUND_POOL_SIZE: usize = 10;
    pub const ENEMY_POOL_SIZE:  usize = 10;

    pub const GROUND_START_IDX: usize = 0;
    pub const GROUND_END_IDX:   usize = Self::GROUND_START_IDX + Self::GROUND_POOL_SIZE - 1;

    pub const ENEMY_START_IDX: usize = Self::GROUND_END_IDX + 1;
    pub const ENEMY_END_IDX:   usize = Self::ENEMY_START_IDX + Self::ENEMY_POOL_SIZE - 1;

    pub const PLAYER_IDX:   usize = Self::ENEMY_END_IDX + 1;
    pub const ENTITY_COUNT: usize = Self::GROUND_POOL_SIZE + Self::ENEMY_POOL_SIZE + 1;

    pub const FONT_START_IDX: usize = Self::PLAYER_IDX + 1;




    pub const QUAD_MESH: usize = 0;

    pub const GROUND_T1_MAT: &'static str = "assets/environment/ground_t1_mat.yaml";
    pub const ENEMY_T1_MAT:  &'static str = "assets/characters/enemy_t1_mat.yaml";
    pub const PLAYER_MAT:    &'static str = "assets/characters/player_mat.yaml";
    pub const FONT_MAT:      &'static str = "assets/fonts/font_bm_fira_code_mat.yaml";

    pub const FONT_FILE_PATH: &str = "assets/fonts/font_bm_fira_code.fnt";

    pub const GROUND_SPAWN_Y:     f32 = Self::FLOOR_Y - Self::GROUND_SIZE;
    pub const GROUND_SPAWN_POS:   glam::Vec3 = glam::Vec3::new(5.0, Self::GROUND_SPAWN_Y, 0.0);
    pub const GROUND_RESET_POS:   glam::Vec3 = glam::Vec3::new(5.0, Self::GROUND_SPAWN_Y, 0.0);

    pub const ENEMY_SPAWN_Y:      f32 = Self::FLOOR_Y;
    pub const ENEMY_SPAWN_POS:    glam::Vec3 = glam::Vec3::new(5.0, Self::ENEMY_SPAWN_Y, 0.0);
    pub const ENEMY_RESET_POS:    glam::Vec3 = glam::Vec3::new(5.0, Self::ENEMY_SPAWN_Y, 0.0);
    pub const ENEMY_KILL_POS_X:   f32 = -5.0;
    pub const WORLD_SCROLL_SPEED: f32 = 5.0;

    pub const GROUND_SPAWN_INTERVAL: f32 = Self::GROUND_SIZE;
    pub const ENEMY_SPAWN_INTERVAL: f32 = 8.0;

    pub const JUMP_FORCE: f32 = 1300.0;
    pub const FALL_FORCE: f32 = -20.0;
}

impl NocoruScene {
    pub fn new() -> Self {

        let scene_data = SceneData::default();
        let render_data = RenderData::default();
        let movement_data = vec![MovementData::default(); Self::ENTITY_COUNT];
        let colliders = vec![AABB2D::default(); Self::ENTITY_COUNT];
        let is_alive = vec![false; Self::ENTITY_COUNT];
        let is_grounded = vec![false; Self::ENTITY_COUNT];

        let gravity_system = GravitySystem::default();
        let movement_system = MovementSystem::default();
        let jump_system = JumpSystem::new(Self::JUMP_FORCE, Self::FALL_FORCE);
        let environment_collision_system = EnvironmentCollisionSystem::new(Self::FLOOR_Y, Self::CEILING_Y);
        let enemy_collision_system = EneymCollisionSystem::default();

        let enemy_spawn_system = EnemySpawnSystem::new(glam::vec2(-Self::WORLD_SCROLL_SPEED, 0.0));
        let enemy_kill_system = EnemyKillSystem::new(Self::ENEMY_KILL_POS_X);

        let score_txt = TextMesh::new(None);

        let font_path = std::path::Path::new(Self::FONT_FILE_PATH);
        let _font_file = FntFile::from_file(font_path).unwrap();

        Self {
            scene_data,
            render_data,
            movement_data,
            colliders,
            is_alive,
            is_grounded,

            gravity_system,
            movement_system,
            jump_system,
            environment_collision_system,
            enemy_collision_system,
            enemy_spawn_system,
            enemy_kill_system,

            ground_distance: 0.0,
            enemy_distance: 0.0,

            score_txt,
        }
    }

    pub fn render_data(&self) -> &RenderData {
        &self.render_data
    }

    pub fn render_data_mut(&mut self) -> &mut RenderData {
        &mut self.render_data
    }

    pub fn reset_scene(&mut self) {
        println!("reset scene");
    }

    pub fn load_scene(&mut self, resource_manager: &mut ResourceManager) -> HellResult<()> {
        // setup environment
        // -----------------
        let ground_t1_mat = resource_manager.load_material(Self::GROUND_T1_MAT)?;
        for _ in Self::GROUND_START_IDX..=Self::GROUND_END_IDX {
            self.render_data.add_data(Self::QUAD_MESH, ground_t1_mat, Transform::default());
        }

        // setup enemies
        // -------------
        let enemy_t1_mat = resource_manager.load_material(Self::ENEMY_T1_MAT)?;
        // let _enemy_t2_mat_idx = resource_manager.load_material(Self::ENEMY_T2_MAT)?;
        for _ in Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX {
            self.render_data.add_data(Self::QUAD_MESH, enemy_t1_mat, Transform::default());
        }

        // setup player
        // ------------
        let player_mat = resource_manager.load_material(Self::PLAYER_MAT)?;
        self.render_data.add_data(Self::QUAD_MESH, player_mat, Transform::default());


        // setup gui
        // ---------
        let font_mat = resource_manager.load_material(Self::FONT_MAT)?;
        let font = HellFont::new(Self::QUAD_MESH, font_mat);
        self.score_txt.set_font(Some(font));
        self.score_txt.set_text("Hell");

        for t in self.score_txt.char_transforms() {
            self.render_data.add_data(Self::QUAD_MESH, font_mat, t.clone());
        }

        // setup systems
        // -------------
        self.enemy_spawn_system.prepare(&Self::GROUND_SPAWN_POS, &mut self.render_data.transforms[Self::GROUND_START_IDX..=Self::GROUND_END_IDX], &mut self.movement_data);
        self.enemy_spawn_system.prepare(&Self::ENEMY_SPAWN_POS, &mut self.render_data.transforms[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX], &mut self.movement_data);

        Ok(())
    }

    pub fn update_scene(&mut self, delta_time: f32, input: &InputManager) -> HellResult<()> {
        let render_data = &mut self.render_data;

        // kill
        // ----
        self.enemy_kill_system.execute(
            &Self::GROUND_RESET_POS,
            &mut render_data.transforms[Self::GROUND_START_IDX..=Self::GROUND_END_IDX],
            &mut self.movement_data[Self::GROUND_START_IDX..=Self::GROUND_END_IDX],
            &mut self.is_alive[Self::GROUND_START_IDX..=Self::GROUND_END_IDX]
        );

        self.enemy_kill_system.execute(
            &Self::ENEMY_RESET_POS,
            &mut render_data.transforms[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX],
            &mut self.movement_data[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX],
            &mut self.is_alive[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX]
        );

        // spwan
        // -----
        if self.ground_distance >= Self::GROUND_SPAWN_INTERVAL {
            let offset = glam::vec3( -(self.ground_distance % Self::GROUND_SPAWN_INTERVAL), 0.0, 0.0);
            self.ground_distance = 0.0;


            let _spawned_ground_idx = self.enemy_spawn_system.execute(
                Self::GROUND_SPAWN_POS + offset,
                &mut render_data.transforms[Self::GROUND_START_IDX..=Self::GROUND_END_IDX],
                &mut self.movement_data[Self::GROUND_START_IDX..=Self::GROUND_END_IDX],
                &mut self.is_alive[Self::GROUND_START_IDX..=Self::GROUND_END_IDX]
            );
        }
        if self.enemy_distance >= Self::ENEMY_SPAWN_INTERVAL {
            let offset = glam::vec3(-(self.enemy_distance % Self::ENEMY_SPAWN_INTERVAL), 0.0, 0.0);

            self.enemy_distance = 0.0;
            let _spawned_enemy_idx = self.enemy_spawn_system.execute(
                Self::ENEMY_SPAWN_POS + offset,
                &mut render_data.transforms[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX],
                &mut self.movement_data[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX],
                &mut self.is_alive[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX]
            );
        }


        self.environment_collision_system.execute(
            &mut render_data.transforms[Self::PLAYER_IDX..=Self::PLAYER_IDX],
            &mut self.movement_data[Self::PLAYER_IDX..=Self::PLAYER_IDX],
            &mut self.is_grounded[Self::PLAYER_IDX..=Self::PLAYER_IDX]
        );

        self.gravity_system.execute(&mut self.movement_data[Self::PLAYER_IDX..=Self::PLAYER_IDX], delta_time);

        let wants_to_jump = input.key_state(KeyCode::Space).is_down();
        self.jump_system.execute(
            delta_time,
            &mut self.movement_data[Self::PLAYER_IDX..=Self::PLAYER_IDX],
            &mut self.is_grounded[Self::PLAYER_IDX..=Self::PLAYER_IDX],
            &[wants_to_jump]
        );

        self.movement_system.execute(delta_time, &mut render_data.transforms[Self::GROUND_START_IDX..=Self::PLAYER_IDX], &self.movement_data)?;
        self.ground_distance += Self::WORLD_SCROLL_SPEED * delta_time;
        self.enemy_distance += Self::WORLD_SCROLL_SPEED * delta_time;

        let did_collide = self.enemy_collision_system.execute(
            &self.colliders[Self::PLAYER_IDX],
            &self.colliders[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX],
            &render_data.transforms[Self::PLAYER_IDX],
            &render_data.transforms[Self::ENEMY_START_IDX..=Self::ENEMY_END_IDX]
        );

        if did_collide {
            self.reset_scene();
        }

        Ok(())
    }
}
