use bevy::prelude::*;

use crate::game::SimulationState;

// pause the game
pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            next_state.set(SimulationState::Paused);
        }
        if *simulation_state.get() == SimulationState::Paused {
            next_state.set(SimulationState::Running);
        }
    }
}