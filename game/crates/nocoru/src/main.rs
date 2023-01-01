// crate-config: start
#![deny(warnings)]
// crate-config: end



mod scene;
mod systems;

use hell_app::HellGame;
use hell_error::HellResult;
use hell_input::InputManager;
use hell_renderer::{shader::SpriteShaderSceneData, render_types::RenderPackage, HellRenderer};
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
        let scene_1 = NocoruScene::new();

        Self {
            scene_1,
        }
    }
}

impl HellGame for NocoruGame {
    fn scene_data(&self)         -> &SpriteShaderSceneData { &self.scene_1.scene_data }
    fn scene_data_mut(&mut self) -> &mut SpriteShaderSceneData  { &mut self.scene_1.scene_data }
    fn render_package(&self)     -> &RenderPackage { self.scene_1.render_pkg() }

    fn init_game(&mut self, renderer: &mut HellRenderer) -> HellResult<()> {
        self.scene_1.load_scene(renderer)?;
        Ok(())
    }

    fn update_game(&mut self, delta_time: f32, input: &InputManager) -> HellResult<()> {
        self.scene_1.update_scene(delta_time, input)
    }
}

