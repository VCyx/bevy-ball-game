
mod events;
mod systems;
mod game;
mod main_menu;

use systems::*;

use bevy::prelude::*;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // My plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Startup systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(exit_game)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}


















