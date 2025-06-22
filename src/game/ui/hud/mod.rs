mod components;
mod styles;
mod systems;

use::bevy::prelude::*;
use systems::layout::*;
use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud);
        // app.add_systems(Update, interact_with_button.run_if(in_state(AppState::GameOver)));
        app.add_systems(OnExit(AppState::Game), despawn_hud);
        
    }
}