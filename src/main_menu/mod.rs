mod styles;
mod systems;
mod components;

use bevy::prelude::*;
use systems::layout::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
    }
}
