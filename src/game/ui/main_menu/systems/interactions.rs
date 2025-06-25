use bevy::prelude::*;
use bevy::app::AppExit;

use crate::AppState;
use super::super::components::*;
use super::super::styles::*;

pub fn interact_with_button(
    // mut button_query: Query<(&Interaction, &mut BackgroundColor, Entity), Changed<Interaction>>,
    // button_play_query: Query<(), With<PlayButton>>,
    // button_quit_query: Query<(), With<QuitButton>>,
    // 
    // or:
    //
    mut button_query: 
        Query<(
                &Interaction, 
                &mut BackgroundColor, 
                // has to be ref to buttons
                Option<&PlayButton>, 
                Option<&QuitButton>
                ), 
            Changed<Interaction>>,
        mut app_state_next: ResMut<NextState<AppState>>,
        mut event_w: EventWriter<AppExit>,
    // 
    // or:
    //
    // use event system later to split ui and logic.
    // -> only interaction query here, no seperate button query
) {
    // for (interaction, mut bg_color, entity) in button_query.iter_mut() {
    for (interaction, mut bg_color, play_b, quit_b) in button_query.iter_mut() {
        // if button_play_query.get(entity).is_ok() {
        if let Some(_) = play_b {
            match *interaction {
                Interaction::Hovered => {
                    *bg_color = UI_HOVERED_BUTTON.into();
                }
                Interaction::Pressed => {
                    // start game
                    app_state_next.set(AppState::Game);
                    *bg_color = UI_PRESSED_BUTTON.into();
                }
                Interaction::None => {
                    *bg_color = UI_PLAY_BUTTON_BG_COL.into();  
                }
            }
        }
        else if let Some(_) = quit_b {
                    match *interaction {
                Interaction::Hovered => {
                    *bg_color = UI_HOVERED_BUTTON.into();
                }
                Interaction::Pressed => {
                    // quit game
                    event_w.write(AppExit::Success);
                    *bg_color = UI_PRESSED_BUTTON.into();
                }
                Interaction::None => {
                    // } else if button_quit_query.get(entity).is_ok() {
                        *bg_color = UI_QUIT_BUTTON_BG_COL.into();
                }
            }
        }
    }
}
