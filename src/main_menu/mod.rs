mod systems;
mod components;
mod styles;

use crate::main_menu::systems::interactions::*;
use crate::main_menu::systems::layout::*;
use crate::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State Systems
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // Systems
            .add_systems(Update,
                (
                    interact_with_play_button,
                    interact_with_quit_button
                    ).run_if(in_state(AppState::MainMenu))
            )
            // On Exit State Systems
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        ;
    }
}

