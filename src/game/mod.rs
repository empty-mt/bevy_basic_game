pub mod enemy;
pub mod player;
pub mod score;
pub mod ui;
mod systems;

use bevy::prelude::*;
use super::events::*;
use super::global_systems::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use score::ScorePlugin;
use ui::UiPlugin;
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
            UiPlugin,
        ))
        // on event condition
        .add_systems(Update, handle_game_over.run_if(on_event::<GameOver>))
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