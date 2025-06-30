use::bevy::prelude::*;
use bevy::reflect::DynamicTyped;

use crate::{events::HudTimeUpdate, game::score::resources::Score};
use super::super::resources::*;
use crate::game::ui::hud::components::{ScoreDisplay, ScoreText, TimeText};

pub fn update_hud_score(
    mut t_query: Query<&mut Text, With<ScoreText>>,
    score: ResMut<Score>,
){
    for mut text in &mut t_query {
        text.0 = score.value.to_string();
    }
}

pub fn update_hud_time(
    mut t_query: Query<&mut Text, With<TimeText>>,
    tick_timer: ResMut<HudTimer>,
){
    for mut text in &mut t_query {
        text.0 = tick_timer.timer.elapsed().as_secs().to_string();
    }
}

pub fn reset_hud_timer(
    mut hud_timer: ResMut<HudTimer>, 
) {
    hud_timer.timer.reset();
}

pub fn unpause_hud_timer(
    mut hud_timer: ResMut<HudTimer>,
){
    hud_timer.timer.unpause();
}

pub fn pause_hud_timer(
    mut hud_timer: ResMut<HudTimer>,
){
    hud_timer.timer.pause();
}
// tick every time.delta()
// and
// send event every elapsed second
pub fn tick_hud_timer(
    mut hud_timer: ResMut<HudTimer>, 
    time: Res<Time>,
    mut event_w: EventWriter<HudTimeUpdate>,
) {
    // tick between
    let before = hud_timer.timer.elapsed_secs() as u32;
    hud_timer.timer.tick(time.delta());
    let after = hud_timer.timer.elapsed_secs() as u32;

    if after > before {
        // event to hud update
        event_w.write(HudTimeUpdate);
    } 
}

pub fn debug_texts(
    query: Query<(Entity, &Text, Option<&ScoreText>, Option<&ScoreDisplay>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for (entity, text, score, display) in &query {
            println!(
                "Entity: {:?}, Text: {:?}, ScoreText: {}, ScoreDisplay: {}",
                entity.reflect_type_info(),
                text,
                score.is_some(),
                display.is_some()
            );
        }
    }
}