mod scene;
mod systems;

use hell_app::HellGame;
use hell_error::HellResult;
use hell_input::{InputManager, KeyCode};
use hell_physics::systems::{GravitySystem, CollisionSystem};
use hell_renderer::render_data::SceneData;
use hell_renderer::vulkan::RenderData;
use hell_resources::ResourceManager;
use hell_winit::Window;

use self::scene::NocoruScene;




fn main() -> HellResult<()> {
    let win = Window::new("hell-app", 800, 600).expect("failed to create window");

    let game = Box::new(NocoruGame::new());
    let leaked_box = Box::leak(game);

    let mut app = hell_app::HellApp::new(&win, leaked_box).expect("failed to create hell-app");
    app.init_game()?; // TODO: change

    win.main_loop(app);

    Ok(())
}




struct NocoruGame {
    scene_1: NocoruScene,
}

impl NocoruGame {
    pub fn new() -> Self {
        let scene_1 = NocoruScene::new_scene_1();

        Self {
            scene_1,
        }
    }
}

impl HellGame for NocoruGame {
    fn scene_data(&self) -> &SceneData               { &self.scene_1.scene_data }
    fn scene_data_mut(&mut self) -> &mut SceneData   { &mut self.scene_1.scene_data }
    fn render_data(&self) -> &RenderData             { &self.scene_1.render_data }
    fn render_data_mut(&mut self) -> &mut RenderData { &mut self.scene_1.render_data }

    fn init_game(&mut self, resource_manager: &mut ResourceManager) -> HellResult<()> {
        self.scene_1.load_scene(resource_manager)?;

        Ok(())
    }

    fn update_game(&mut self, delta_time: f32, input: &InputManager) -> HellResult<()> {
        self.scene_1.update_scene_1(delta_time, input)
    }
}

