use bevy::prelude::*;

pub const HUD_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct HudTimer {
    pub timer: Timer,
}

impl Default for HudTimer {
    fn default() -> HudTimer {
        HudTimer {
            timer: Timer::from_seconds(20.0, TimerMode::Once)
            // timer: Timer::from_seconds(HUD_TIME, TimerMode::Repeating)
        }
    }
}