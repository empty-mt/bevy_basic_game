pub mod components;
mod styles;
mod systems;

use::bevy::prelude::*;
use systems::{
    layout::*, 
    updates::*
};
use crate::AppState;
use crate::game::SimulationState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud);
        // app.add_systems(OnEnter(SimulationState::Paused), spawn_hud);
        app.add_systems(Update, 
            update_hud_score
            .run_if(in_state(AppState::Game))
            .after(spawn_hud)
            );

        // debug
        app.add_systems(Update, debug_texts);
        //
        app.add_systems(OnEnter(AppState::GameOver), despawn_hud);
        app.add_systems(OnEnter(AppState::MainMenu), despawn_hud);
        // app.add_systems(OnExit(AppState::Game), despawn_hud);
        
    }
}