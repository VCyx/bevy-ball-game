use bevy::prelude::*;
use crate::game::SimulationState;

pub fn pause_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>
) {
   println!("Pause");
    simulation_state_next_state.set(SimulationState::Paused)
}

pub fn resume_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>
) {
    simulation_state_next_state.set(SimulationState::Running)
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Running => simulation_next_state.set(SimulationState::Paused),
            SimulationState::Paused => simulation_next_state.set(SimulationState::Running),
        }
    }
}
