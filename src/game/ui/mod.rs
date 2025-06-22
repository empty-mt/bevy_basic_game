mod game_over_menu;
mod hud;
mod pause_menu;

use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use pause_menu::PauseMenuPlugin;
use hud::HudPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            (
            HudPlugin,
            PauseMenuPlugin,
            GameOverMenuPlugin,
        ));
    }
}