use bevy::prelude::*;

// use crate::score::resources::*;
// or:
use super::resources::*;

pub fn print_score(score: Res<Score>) {
    if score.is_changed() {
        println!("{:?}", score.value);
    }
}