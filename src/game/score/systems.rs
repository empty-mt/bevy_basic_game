use bevy::prelude::*;

use super::SCORE_MAX;
use crate::events::LevelUp;
use crate::AppState;

// use crate::score::resources::*;
// or:
use super::resources::*;

// pub fn print_score(score: Res<Score>) {
//     if score.is_changed() {
//         println!("{:?}", score.value);
//     }
// }

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

pub fn reset_score(
    mut score_query: ResMut<Score>,
) {
    if score_query.value >= SCORE_MAX {
        score_query.value = 0;
    } 
    // commands.remove_resource::<Score>();
}

// lvl up event on max score
pub fn send_max_score_event(
    score_query: ResMut<Score>,
    mut level_up_event: EventWriter<LevelUp>,
) {
    if score_query.value >= SCORE_MAX {
        level_up_event.write(LevelUp);
    }
}

// state transition on level up
pub fn handle_level_up(
    mut level_up_event_r: EventReader<LevelUp>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _event in level_up_event_r.read() {
        next_state.set(AppState::LevelUpMenu);
    }
}
