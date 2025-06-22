use bevy::prelude::*;

use crate::main_menu::components::{PlayButton, QuitButton};
use crate::main_menu::styles::*;

pub fn interact_with_button(
    // mut button_query: Query<(&Interaction, &mut BackgroundColor, Entity), Changed<Interaction>>,
    // button_play_query: Query<(), With<PlayButton>>,
    // button_quit_query: Query<(), With<QuitButton>>,
    // 
    // or:
    //
    mut button_query: Query<(
        &Interaction, 
        &mut BackgroundColor, 
        // has to be ref to buttons
        Option<&PlayButton>, 
        Option<&QuitButton>
    ), 
        Changed<Interaction>>,
    // 
    // or:
    //
    // use event system later to split ui and logic.
    // -> only interaction query here, no seperate button query
) {
    // for (interaction, mut bg_color, entity) in button_query.iter_mut() {
    for (interaction, mut bg_color, play_b, quit_b) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = UI_HOVERED_BUTTON.into();
            }
            Interaction::Pressed => {
                *bg_color = UI_PRESSED_BUTTON.into();
            }
            Interaction::None => {
                // if button_play_query.get(entity).is_ok() {
                if let Some(_) = play_b {
                    *bg_color = UI_PLAY_BUTTON_BG_COL.into();
                }
                // } else if button_quit_query.get(entity).is_ok() {
                if let Some(_) = quit_b {
                    *bg_color = UI_QUIT_BUTTON_BG_COL.into();
                }
            }
        }
    }
}