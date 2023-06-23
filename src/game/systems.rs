use bevy::prelude::*;

use crate::game::GameState;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<GameState>>) {
    simulation_state_next_state.set(GameState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<GameState>>) {
    simulation_state_next_state.set(GameState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<GameState>>,
    mut simulation_state_next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == GameState::Running {
            simulation_state_next_state.set(GameState::Paused);
            println!("Simulation Paused.");
        }
        if simulation_state.0 == GameState::Paused {
            simulation_state_next_state.set(GameState::Running);
            println!("Simulation Running.");
        }
    }
}
