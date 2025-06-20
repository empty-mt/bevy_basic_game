// using 0.10 guide
// https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=2

mod global_systems;
mod global_plugins;
mod sys_sets;
mod game;
mod main_menu;
mod events;

use bevy::prelude::*;
use global_plugins::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    let mut app = App::new();
    
    app
    .add_plugins(CustomPlugin)
    .run();
}

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(DefaultPlugins)
        .insert_state(AppState::default())
        // .insert_state::<AppState>()
        .add_plugins((
            MainMenuPlugin,
            GamePlugin,
            GlobalPlugin,
        ));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
