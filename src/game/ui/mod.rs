mod game_over_menu;
pub mod hud;
mod pause_menu;
mod main_menu;
pub mod level_up_menu;

use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use pause_menu::PauseMenuPlugin;
use main_menu::MainMenuPlugin;
use level_up_menu::LevelUpMenuPlugin;
use hud::HudPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            (
            MainMenuPlugin,
            HudPlugin,
            PauseMenuPlugin,
            GameOverMenuPlugin,
            LevelUpMenuPlugin,
        ));
    }
}