// declare submodules here
pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;
// use resources::*;

pub const PLAYER_SIZE: f32 = 64.0;      // player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const SCORE_MAX: i32 = 10;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) { 
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, player_movement)
        .add_systems(Update, enemy_hit_player);
    }
}