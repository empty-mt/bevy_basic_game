use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::audio::Volume;
use bevy::window::PrimaryWindow;

use crate::events::*;
use crate::game::SimulationState;
use crate::AppState;
use crate::game::enemy::components::Enemy;

pub const GLOBAL_VOLUME: f32 = 0.1;     // [0.0 - 1.0]

//
// setup
//

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

//
// actions
//

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
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _event in game_over_event_r.read() {
        next_state.set(AppState::GameOver);
    }
}

//
// pause game
//

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::Game {
            next_state.set(AppState::Game);
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,

) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            next_state.set(AppState::MainMenu);
            next_sim_state.set(SimulationState::Running);
        }
    }
}

//
// debug
//

pub fn print_appstate(
    mut events: EventReader<StateTransitionEvent<AppState>>,
) {
    for event in events.read() {
        println!("{:?}", event);
    }
}

pub fn print_simulationstate(
    mut events: EventReader<StateTransitionEvent<SimulationState>>,
) {
    for event in events.read() {
        println!("{:?}", event);
    }
}

pub fn force_enemy_despawn(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyO) {
        for enemy_entity in enemy_query.iter() {
            commands.entity(enemy_entity).despawn();
        }
    }
}

//
// UI stuff
//

pub fn ui_create_rounded_rect_button_node() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(6.0)),
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    }
}



