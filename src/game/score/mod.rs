// declare submodules here

pub mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;
use resources::*;
use crate::events::LevelUp;

use crate::AppState;


const SCORE_MAX: i32 = 5;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) { 
        app
        .init_resource::<Score>() 
        // .add_systems(Startup, insert_score)
        .add_systems(OnEnter(AppState::Game), insert_score)
        .add_systems(Update, send_max_score_event.run_if(in_state(AppState::Game)))
        .add_systems(Update, handle_level_up.run_if(on_event::<LevelUp>))

        // .add_systems(Update, print_score.run_if(in_state(AppState::Game)))
        // one time rm -> one score
        .add_systems(OnExit(AppState::Game), reset_score)
        // .add_systems(OnExit(AppState::Game), remove_score)
        ;
    }
}