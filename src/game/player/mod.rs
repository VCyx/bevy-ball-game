use bevy::prelude::*;
use crate::AppState;
use crate::game::player::systems::*;
use crate::game::SimulationState;

mod components;
mod systems;

pub const PLAYER_SIZE: f32 = 64.0; // This is player sprite size
pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct MovementSystemSet;
//
// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct ConfinementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .configure_sets(Update, PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // .add_systems((player_movement, confine_player_movement).chain())
            .add_systems(Update,
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    confine_player_movement.in_set(PlayerSystemSet::Confinement)
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(Update, (
                enemy_hit_player,
                player_hit_star
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_player)
        ;
    }
}
