// declare submodules here


pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;
use resources::*;

pub const ENEMY_SIZE: f32 = 64.0;       // nmy sprite size
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_NUM: i16 = 16;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) { 
        app
        .init_resource::<EnemyTimer>() 
        // .add_systems(Startup, spawn_enemies)
        // .add_systems(Update, despawn_enemies_over_time)
        // .add_systems(Update, spawn_enemies)
        // .add_systems(Update, update_enemy_movement)
        // .add_systems(Update, confine_enemy_movement);
        .add_systems(Update, tick_despawn_enemy_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, (enemy_movement, update_enemy_movement, confine_enemy_movement).chain());
    }
}