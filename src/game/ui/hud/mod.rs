pub mod components;
pub mod styles;
mod systems;
mod resources;

use::bevy::prelude::*;
use systems::{
    layout::*, 
    updates::*
};
use crate::AppState;
use resources::*;
use crate::events::HudTimeUpdate;
use crate::game::SimulationState;
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HudTimer>();
        app.add_systems(OnEnter(AppState::Game), spawn_hud);
        app.add_systems(OnEnter(AppState::GameOver), despawn_hud);
        app.add_systems(OnEnter(AppState::MainMenu), despawn_hud);
        app.add_systems(Update, update_hud_time.run_if(on_event::<HudTimeUpdate>));
        app.add_systems(Update, (update_hud_score, tick_hud_timer)
            .run_if(in_state(AppState::Game))
            .after(spawn_hud)
        );
        // timer
        app.add_systems(OnEnter(SimulationState::Running), unpause_hud_timer);
        app.add_systems(OnEnter(SimulationState::Paused), pause_hud_timer);
        app.add_systems(OnExit(AppState::Game), reset_hud_timer);
        // debug
        app.add_systems(Update, debug_texts);
    }
}