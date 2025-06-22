use bevy::prelude::*;

use crate::game::ui::game_over_menu::components::{RestartButton, MainMenuButton};
use crate::game::ui::game_over_menu::styles::*;
use crate::AppState;

pub fn interact_with_button(
    // mut button_query: Query<(&Interaction, &mut BackgroundColor, Entity), Changed<Interaction>>,
    // button_play_query: Query<(), With<PlayButton>>,
    // button_quit_query: Query<(), With<MainMenuButton>>,
    // 
    // or:
    //
    mut button_query: 
        Query<(
                &Interaction, 
                &mut BackgroundColor, 
                // has to be ref to buttons
                Option<&RestartButton>, 
                Option<&MainMenuButton>
                ), 
            Changed<Interaction>>,
        mut app_state_next: ResMut<NextState<AppState>>,
    // 
    // or:
    //
    // use event system later to split ui and logic.
    // -> only interaction query here, no seperate button query
) {
    // for (interaction, mut bg_color, entity) in button_query.iter_mut() {
    for (interaction, mut bg_color, restart_b, mmenu_b) in button_query.iter_mut() {
        // if button_play_query.get(entity).is_ok() {
        if let Some(_) = restart_b {
            match *interaction {
                Interaction::Hovered => {
                    *bg_color = UI_HOVERED_BUTTON.into();
                }
                Interaction::Pressed => {
                    // resume game
                    app_state_next.set(AppState::Game);
                    *bg_color = UI_PRESSED_BUTTON.into();
                }
                Interaction::None => {
                    *bg_color = UI_RESTART_BUTTON_BG_COL.into();  
                }
            }
        }
        else if let Some(_) = mmenu_b {
                    match *interaction {
                Interaction::Hovered => {
                    *bg_color = UI_HOVERED_BUTTON.into();
                }
                Interaction::Pressed => {
                    // go back to main menu
                    app_state_next.set(AppState::MainMenu);
                    *bg_color = UI_PRESSED_BUTTON.into();
                }
                Interaction::None => {
                    // } else if button_quit_query.get(entity).is_ok() {
                        *bg_color = UI_MAIN_MENU_BUTTON_BG_COL.into();
                }
            }
        }
    }
}
