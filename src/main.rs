// using 0.10 guide
// https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=2

mod events;
mod global_systems;
mod global_plugins;
mod enemy;
mod player;
mod score;

use bevy::prelude::*;
use global_plugins::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use score::ScorePlugin;

fn main() {
    let mut app = App::new();
    
    app
    .add_plugins(DefaultPlugins)
    .add_plugins(CustomPlugin)
    .run();
}

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        app    
        .add_plugins((
            GlobalPlugin,
            PlayerPlugin,
            EnemyPlugin,
            ScorePlugin
        ));
    }
}
