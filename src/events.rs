use bevy::prelude::*;

// init game events in game/mod

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

#[derive(Event)]
pub struct LevelUp;
