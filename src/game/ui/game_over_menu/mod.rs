mod systems;
mod components;
mod styles;

use::bevy::prelude::*;


pub struct GameOverMenuPlugin;
use crate::AppState;
use systems::{
    interactions::*, 
    layout::*
};

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu);
        app.add_systems(Update, interact_with_button.run_if(in_state(AppState::GameOver)));
        app.add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}