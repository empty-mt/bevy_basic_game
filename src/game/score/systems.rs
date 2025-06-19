use bevy::prelude::*;

// use crate::score::resources::*;
// or:
use super::resources::*;

pub fn print_score(score: Res<Score>) {
    if score.is_changed() {
        println!("{:?}", score.value);
    }
}

pub fn insert_score(
    mut commands: Commands,
) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(
    mut commands: Commands,
) {
    commands.remove_resource::<Score>();
}