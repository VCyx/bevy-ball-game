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
            .configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            // On Enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // .add_systems((player_movement, confine_player_movement).chain())
            .add_systems(
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    confine_player_movement.in_set(PlayerSystemSet::Confinement)
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
            )
            .add_systems((
                enemy_hit_player,
                player_hit_star
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            )
            // On Exit State
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
        ;
    }
}
