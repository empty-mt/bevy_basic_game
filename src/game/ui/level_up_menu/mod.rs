mod systems;

mod components;
mod styles;

use::bevy::prelude::*;

use crate::AppState::LevelUpMenu;
use systems::{
    interactions::*, 
    layout::*
};
pub struct LevelUpMenuPlugin;

impl Plugin for LevelUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(LevelUpMenu), spawn_level_up_menu)
        // .add_systems(Update, handle_level_up.run_if(not(in_state(LevelUpMenu))))
        .add_systems(Update, interact_with_button.run_if(in_state(LevelUpMenu)))
        .add_systems(OnExit(LevelUpMenu), despawn_level_up_menu)
        ;
    }
}