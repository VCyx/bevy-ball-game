pub mod enemy;
mod player;
mod score;
mod star;
mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use crate::AppState;
use crate::events::GameOver;
use crate::game::systems::{pause_simulation, resume_simulation, toggle_simulation};

pub struct GamePlugin;


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // On Enter Systems
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // On Exit Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
        ;
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
