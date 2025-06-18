// declare submodules here
pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;
use super::super::sys_sets::*;
// use resources::*;

pub const PLAYER_SIZE: f32 = 64.0;      // player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const SCORE_MAX: i32 = 30;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) { 
        app
        // using system set:
        //
        // configure schedule and sets+order in configure_sets
        .configure_sets(Update, (PlayerSet::Movement, PlayerSet::Confinement).chain()) 
        // add systems from configured sets
        .add_systems(Update, confine_player_movement.in_set(PlayerSet::Confinement))
        .add_systems(Update, player_movement.in_set(PlayerSet::Movement))
        // to do:
        //
        // .add_systems(Update, (player_movement, confine_player_movement).chain())
        
        .add_systems(Startup, spawn_player)
        .add_systems(Update, enemy_hit_player);
    }
}