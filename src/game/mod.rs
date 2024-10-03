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
            .init_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // On Enter Systems
            .add_systems(OnEnter(AppState::Game), resume_simulation)
            // Plugins
            .add_plugins((
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin
            ))
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // On Exit Systems
            .add_systems(OnExit(AppState::Game), pause_simulation)
        ;
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
