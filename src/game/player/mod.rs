// declare submodules here
pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;
use crate::{game::SimulationState, AppState};

use super::super::sys_sets::*;

pub const PLAYER_SIZE: f32 = 64.0;      // player sprite size
pub const PLAYER_SPEED: f32 = 500.0;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) { 
        app
        // using system set:
        //
        // player movement: schedule and sets+orders+conditions:
        .configure_sets(
            Update, 
            (PlayerSet::Collision, PlayerSet::Movement, PlayerSet::Confinement)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
            .chain()) 
        // add systems from configured sets:
        .add_systems(Update, confine_player_movement.in_set(PlayerSet::Confinement))
        .add_systems(Update, player_movement.in_set(PlayerSet::Movement))
        .add_systems(Update, enemy_hit_player.in_set(PlayerSet::Collision))
        // equals:
        //
        // .add_systems(Update, (player_movement, confine_player_movement)
        // .run_if(...).run_if(...).chain())

        // state systems
        .add_systems(OnEnter(AppState::Game), spawn_player)
        // one time, because we have one player
        .add_systems(OnExit(AppState::Game), despawn_player)

        // gameplay systems
        // .add_systems(Update, enemy_hit_player)
        ;
    }
}