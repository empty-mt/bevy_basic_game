use::bevy::prelude::*;
use bevy::reflect::DynamicTyped;

use crate::game::score::resources::Score;
use crate::game::ui::hud::components::{ScoreText, ScoreDisplay};

pub fn update_hud_score(
    mut t_query: Query<&mut Text, With<ScoreText>>,
    score: ResMut<Score>,
){
    for mut text in &mut t_query {
        text.0 = score.value.to_string();
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