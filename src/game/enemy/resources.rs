use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 0.5;

#[derive(Resource)]
pub struct EnemyTimer {
    pub timer: Timer,
}

impl Default for EnemyTimer {
    fn default() -> EnemyTimer {
        EnemyTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating)
        }
    }
}