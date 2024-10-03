
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
        .init_state::<AppState>()
        // My plugins
        .add_plugins((
            MainMenuPlugin,
            GamePlugin
        ))
        // Startup systems
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(Update, (
            exit_game,
            transition_to_game_state,
            transition_to_main_menu_state,
            handle_game_over
        ))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}


















