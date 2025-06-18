// declare submodules here

pub mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;
use resources::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) { 
        app
        .init_resource::<Score>() 
        .add_systems(Update, print_score);
    }
}