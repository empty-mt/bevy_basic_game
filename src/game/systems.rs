use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    // mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            // commands.insert_resource(NextState(Some(SimulationState::Paused)));
            next_state.set(SimulationState::Paused);
            println!("paused.")
        }
        if *simulation_state.get() == SimulationState::Paused {
            next_state.set(SimulationState::Running);
            println!("running.")
        }

    }
}