use bevy::prelude::*;
use bevy::audio::Volume;
use super::global_systems::*;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app    
        // global volume
        .insert_resource(GlobalVolume::new(Volume::Decibels(GLOBAL_VOLUME)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, max_window)
        .add_systems(Startup, change_global_volume)
        
        .add_systems(Update, exit_game);
    }
}

