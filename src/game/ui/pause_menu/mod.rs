mod systems;
mod components;
mod styles;

use::bevy::prelude::*;

use crate::game::SimulationState;
use systems::{
    interactions::*, 
    layout::*
};
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu);
        app.add_systems(Update, interact_with_button.run_if(in_state(SimulationState::Paused)));
        app.add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}