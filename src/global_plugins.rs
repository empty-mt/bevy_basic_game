use bevy::prelude::*;
use bevy::audio::Volume;

use super::global_systems::*;
use crate::AppState;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app    
        // global volume
        .insert_resource(GlobalVolume::new(Volume::Decibels(GLOBAL_VOLUME)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, max_window)
        .add_systems(Startup, change_global_volume)
        // debug
        .add_systems(Update, print_appstate.run_if(state_changed::<AppState>))
        .add_systems(Update, force_enemy_despawn.run_if(in_state(AppState::Game)))
        //
        .add_systems(Update, (transition_to_game_state, transition_to_main_menu_state))
        .add_systems(Update, exit_game);
    }
}

