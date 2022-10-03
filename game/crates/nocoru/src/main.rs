use hell_app::HellGame;
use hell_app::scene::Scene;
use hell_common::HellResult;
use hell_common::transform::Transform;
use hell_resources::ResourceManager;




fn main() {
    let win = hell_winit::WinitWindow::new("hell-app", 800, 600).expect("failed to create window");

    let game = Box::new(NocoruGame::new());
    let leaked_box = Box::leak(game);

    let mut app = hell_app::HellApp::new(&win, leaked_box).unwrap();

    let scene = app.create_scene();
    app.load_scene(scene).unwrap();

    win.main_loop(app);
}




struct NocoruGame {
    player_mat: usize,
    enemy_1_mat: usize,
    enemy_2_mat: usize,
}

impl NocoruGame {
    pub fn new() -> Self {
        Self {
            player_mat: usize::MAX,
            enemy_1_mat: usize::MAX,
            enemy_2_mat: usize::MAX,
        }
    }
}

impl HellGame for NocoruGame {

    fn init_game(&mut self, scene: &mut Scene, resource_manager: &mut ResourceManager) -> HellResult<()> {
        let _ = resource_manager.load_image("assets/player_tex.png", true)?;
        let _ = resource_manager.load_image("assets/enemy_1_tex.png", true)?;
        let _ = resource_manager.load_image("assets/enemy_2_tex.png", true)?;

        self.player_mat  = resource_manager.load_material("assets/player_mat.yaml")?;
        self.enemy_1_mat = resource_manager.load_material("assets/enemy_1_mat.yaml")?;
        self.enemy_2_mat = resource_manager.load_material("assets/enemy_2_mat.yaml")?;

        let render_data = scene.get_render_data_mut();
        render_data.add_data(0, self.player_mat, Transform::default());
        render_data.add_data(0, self.enemy_1_mat, Transform::default());
        render_data.add_data(0, self.enemy_2_mat, Transform::default());

        Ok(())
    }

    fn update_game(&mut self, scene: &mut Scene, delta_time: f32) -> HellResult<()> {
        let render_data = scene.get_render_data_mut();

        let trans_1 = &mut render_data.transforms[0];
        trans_1.scale_uniform(1f32 + delta_time / 5f32);
        trans_1.rotate_around_z((delta_time * 30f32).to_radians());
        trans_1.translate_x(delta_time);

        let trans_2 = &mut render_data.transforms[1];
        trans_2.scale_uniform(1f32 - delta_time / 5f32);
        trans_2.rotate_around_z((delta_time * -30f32).to_radians());
        trans_2.translate_x(-delta_time);

        Ok(())
    }
}

