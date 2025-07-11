use bevy::prelude::*;

use crate::game::ui::level_up_menu::components::{ContinueButton, MainMenuButton};
use crate::game::ui::level_up_menu::styles::*;
use crate::game::SimulationState;
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
                Option<&ContinueButton>, 
                Option<&MainMenuButton>
                ), 
            Changed<Interaction>>,
        mut app_state_next: ResMut<NextState<AppState>>,
        mut sim_state_next: ResMut<NextState<SimulationState>>,
    // 
    // or:
    //
    // use event system later to split ui and logic.
    // -> only interaction query here, no seperate button query
) {
    // for (interaction, mut bg_color, entity) in button_query.iter_mut() {
    for (interaction, mut bg_color, continue_b, main_menu_b) in button_query.iter_mut() {
        // if button_play_query.get(entity).is_ok() {
        if let Some(_) = continue_b {
            match *interaction {
                Interaction::Hovered => {
                    *bg_color = UI_HOVERED_BUTTON.into();
                }
                Interaction::Pressed => {
                    // resume game
                    *bg_color = UI_PRESSED_BUTTON.into();
                    app_state_next.set(AppState::Game);
                    sim_state_next.set(SimulationState::Running);
                }
                Interaction::None => {
                    *bg_color = UI_CONTINUE_BUTTON_BG_COL.into();  
                }
            }
        }
        else if let Some(_) = main_menu_b {
            match *interaction {
                Interaction::Hovered => {
                    *bg_color = UI_HOVERED_BUTTON.into();
                }
                Interaction::Pressed => {
                    // go back to main menu
                    *bg_color = UI_PRESSED_BUTTON.into();
                    app_state_next.set(AppState::MainMenu);
                    // sim_state_next.set(SimulationState::Running);
                }
                Interaction::None => {
                    // } else if button_quit_query.get(entity).is_ok() {
                        *bg_color = UI_MAIN_MENU_BUTTON_BG_COL_2.into();
                }
            }
        }
    }
}

