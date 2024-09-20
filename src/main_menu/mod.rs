mod systems;
mod components;
mod styles;

use bevy::prelude::*;
use crate::AppState;
use crate::main_menu::systems::layout::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State Systems
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            // On Exit State Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
        ;
    }
}

