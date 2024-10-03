use bevy::prelude::*;
use crate::AppState;
use crate::game::SimulationState;
use crate::game::star::resources::*;
use crate::game::star::systems::*;

pub mod components;
mod resources;
mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(Update,
                         (
                             tick_star_spawn_timer,
                             spawn_stars_over_time
                         )
                             .run_if(in_state(AppState::Game))
                             .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_stars)
        ;
    }
}
