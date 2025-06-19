pub mod enemy;
pub mod player;
pub mod score;
mod systems;

use bevy::prelude::*;
use super::events::*;
use super::global_systems::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use score::ScorePlugin;
use systems::*;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        
        app
        .insert_state(SimulationState::default())
        .add_event::<GameOver>()
        .add_plugins((            
            PlayerPlugin,
            EnemyPlugin,
            ScorePlugin,
        ))
        .add_systems(Update, handle_game_over)
        .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
        ;
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running, 
    Paused,
}