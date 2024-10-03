use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::events::GameOver;
use crate::game::SimulationState;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyG) {
        match app_state.get() {
            AppState::Game => {}
            AppState::MainMenu => {
                next_app_state.set(AppState::Game);
                println!("Entered AppState::Game")
            }
            AppState::GameOver => {
                next_app_state.set(AppState::MainMenu);
                next_simulation_state.set(SimulationState::Paused);
                println!("Entered AppState::MainMenu")
            }
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyM) {
        match app_state.get() {
            AppState::MainMenu => {}
            AppState::Game => {
                next_app_state.set(AppState::MainMenu);
                next_simulation_state.set(SimulationState::Paused);
                println!("Entered AppState::MainMenu")
            }
            AppState::GameOver => {
                next_app_state.set(AppState::MainMenu);
                next_simulation_state.set(SimulationState::Paused);
                println!("Entered AppState::MainMenu")
            }
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score);
        next_state.set(AppState::GameOver);
        next_state.set(AppState::MainMenu);
    }
}
