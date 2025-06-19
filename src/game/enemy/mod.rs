// declare submodules here
pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;
use resources::*;
use crate::{game::SimulationState, AppState};
use crate::sys_sets::EnemySet;

pub const ENEMY_SIZE: f32 = 64.0;       // nmy sprite size
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_NUM: i16 = 16;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) { 
        app
        .init_resource::<EnemyTimer>() 
        // .add_systems(Startup, spawn_enemies)
        //
        // using set:
        //
        // declare conditions for set:
        .configure_sets(
            Update, 
            EnemySet
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running)))
        // ..and add systems to set:
        .add_systems(
            Update, 
            (spawn_enemies_over_time, tick_spawn_enemy_timer)
            .in_set(EnemySet))
        // equals:
        //
        // .add_systems(
        //     Update, 
        //     (spawn_enemies_over_time, tick_spawn_enemy_timer)
        //     .run_if(in_state(AppState::Game))
        //     .run_if(in_state(SimulationState::Running)))
        //

        // chain systems for clean movement
        .add_systems(
            Update, 
            (enemy_movement, update_enemy_movement, confine_enemy_movement)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
            .chain())
        
        // 
        .add_systems(Update, despawn_enemies.run_if(in_state(AppState::MainMenu)))
        ;
    }
}