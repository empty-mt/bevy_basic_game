pub mod enemy;
pub mod player;
pub mod score;

use bevy::prelude::*;
use super::events::*;
use super::global_systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        
        app
        .add_event::<GameOver>()
        .add_systems(Startup, game)
        .add_systems(Update, handle_game_over);

    }
}

pub fn game() {
    println!("in game");
}