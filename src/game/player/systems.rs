// use crate::game::player::resources::*;
use crate::game::player::components::*;
use crate::game::score::resources::*;
use crate::game::enemy::components::*;
use crate::events::*;
use crate::game::enemy::{ENEMY_SIZE};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::*;

pub fn despawn_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
) {
    if let Ok((player_entity, _player_transform)) = player_query.single_mut() {
        commands.entity(player_entity).despawn();
        println!("deleted player");
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window = window_query.single().unwrap();
    let vec = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/tile_0004.png"), 
            // image_mode: Auto,
            // rect: Some(Rect::from_center_size(Vec2::ZERO, Vec2::ONE)), 
            ..default() },
        Transform::from_translation(vec),
        Player {},

    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) { direction += Vec3::new(-1.0, 0.0, 0.0) }      
        if keyboard_input.pressed(KeyCode::KeyD) { direction += Vec3::new(1.0, 0.0, 0.0) }      
        if keyboard_input.pressed(KeyCode::KeyW) { direction += Vec3::new(0.0, 1.0, 0.0) }      
        if keyboard_input.pressed(KeyCode::KeyS) { direction += Vec3::new(0.0, -1.0, 0.0) }
        // vector normalization for diagonals      
        direction = direction.normalize_or_zero();
        
        //
        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

// constraints for moving player out of window
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = window_query.single().unwrap();
        // why "size/8" is necessary -> sprite issue?
        let eighth_size = PLAYER_SIZE / 8.0;
        let x_min = (window.width()/-2.0) + eighth_size;
        let x_max = (window.width()/2.0) - eighth_size;
        let y_min = (window.height()/-2.0) + eighth_size;
        let y_max = (window.height()/2.0) - eighth_size;
        let mut translation = player_transform.translation;

        // constraint for player x and y pos
        translation.x = translation.x.clamp(x_min, x_max);
        translation.y = translation.y.clamp(y_min, y_max);
        // set new position
        player_transform.translation = translation;
    }
    
}

// check if pixel of player and nmy are overlapping
pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_w: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    // mut enemy_query: Query<&Transform, With<Enemy>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok((_player_entity, player_transform)) = player_query.single_mut() {
        // for enemy_transform in enemy_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {

            let dist = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 8.;
            let enemy_radius = ENEMY_SIZE / 8.;

            // despawn enemy
            if dist < player_radius + enemy_radius {
                let sound = asset_server.load("audio/pluck_002.ogg");
                
                commands.spawn(AudioPlayer::<AudioSource>(sound));
                commands.entity(enemy_entity).despawn();
                score.value +=1;
            }

            // win condition
            // 
            if score.value == SCORE_MAX {
                game_over_event_w.write(GameOver { score: score.value as u32 });
                // commands.entity(player_entity).despawn();
                // app_exit_event_w.write(AppExit::Success);
            }
        }
    }
}