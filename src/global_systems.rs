use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::audio::Volume;
use bevy::window::PrimaryWindow;

use crate::events::*;

pub const GLOBAL_VOLUME: f32 = 0.1;     // [0.0 - 1.0]

pub fn max_window(windows: Query<&mut Window>) {
    for mut window in windows {
        window.set_maximized(true);
    }
}

pub fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
    volume.volume = Volume::Linear(GLOBAL_VOLUME);
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let _window = window_query.single().unwrap();

    commands.spawn(Camera2d{});
}

// close app
pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_w: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_w.write(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut game_over_event_r: EventReader<GameOver>,
) {
    for event in game_over_event_r.read() {
        println!("reached goal of {:?} kills.", event.score.to_string());
    }
}
