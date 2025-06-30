use crate::game::enemy::resources::*;
use crate::game::enemy::components::*;
// or:
// use super::components::*; 

use bevy::prelude::*;
use rand::Rng;
use bevy::window::PrimaryWindow;
use crate::game::ui::hud::components::*;

// using const in enemy/mod.rs
use super::*;

// pub fn spawn_enemies(
//     // keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.single().unwrap();

//     // if keyboard_input.just_pressed(KeyCode::KeyK) {
//         for _ in 0..ENEMY_NUM {
//             let y = rand::rng().random_range(-1.0..1.0) * window.height() / 2.;
//             let x = rand::rng().random_range(-1.0..1.0) * window.width() / 2.;
//             // !! max_window does not upscale window.size()
//             //
//             // let vec = Vec3::new(x, y, 0.0);
//             // println!("{} {} == {:?} || {:?}",x,y,vec, window.size());

//             commands.spawn((
//                 Sprite::from_image(asset_server.load("sprites/tile_0005.png")),
//                 // Sprite {
//                 //     image: asset_server.load("sprites/tile_0005.png"), 
//                 //     // image_mode: Auto,
//                 //     // rect: Some(Rect::from_center_size(Vec2::new(x, y), Vec2::ONE)), 
//                 //     ..default() },
//                 Transform::from_xyz(x, y, 0.0),
//                 // Transform::from_translation(vec),
//                 Enemy {
//                     direction: Vec3::new(
//                         // rand::rng().random_range(-1..1) as f32,
//                         // rand::rng().random_range(-1..1) as f32,
//                         (rand::rng().random_range(0..2) * 2 - 1) as f32,
//                         (rand::rng().random_range(0..2) * 2 - 1) as f32,
//                         0.
//                     ).normalize_or_zero(),
//                 },
//             ));
//         }
//     // }
// }

// enemy hits wall
pub fn update_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    hud_query: Query<&Node, With<Hud>>,
) {
    if let Ok(window) = window_query.single() {
        
        // prevent player from moving "under" ui
        if let Ok(hud_node) = hud_query.single() {
            
            let hud_offset = hud_node.height.resolve(window.height(), Vec2::ONE);
            // why "size/8" is necessary -> sprite issue?
            let eighth_size = ENEMY_SIZE / 8.0;
            let x_min = (window.width()/-2.0) + eighth_size;
            let x_max = (window.width()/2.0) - eighth_size;
            let y_min = (window.height()/-2.0) + eighth_size;
            let y_max = (window.height()/2.0) - eighth_size - hud_offset.unwrap();
            
            for (transform, mut enemy) in enemy_query.iter_mut() {
                    // get actual position as vec3
                    let translation = transform.translation;
                    // indicator for audio
                    let mut dir_changed: bool = false; 
                    
                    // flip direction if hitting edges
                    if translation.x < x_min || translation.x > x_max { 
                        dir_changed = true;
                        enemy.direction.x *= -1.; 
                    }
                    if translation.y < y_min || translation.y > y_max { 
                        dir_changed = true;
                        enemy.direction.y *= -1.; 
                    }
                    
                    // play audio 
                    if dir_changed { 
                        let sound = asset_server.load("audio/pluck_001.ogg");
                    // let mut player = AudioPlayer::new(sound);
                    commands.spawn(AudioPlayer::<AudioSource>(sound));
                    // reduce sound 
                    // commands.spawn((player, music_controller.set_volume(Volume::Decibels(-0.5))));
                    // music_controller.set_volume(Volume::Decibels(-0.5));
                }
            }
        }
    }  
}

// constraints for enemy moving out of window
pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    hud_query: Query<&Node, With<Hud>>,
) {
    let window = match window_query.single() {
        Ok(a) => a,
        Err(_) => {return;}
    }; 
    // prevent player from moving "under" ui
    if let Ok(hud_node) = hud_query.single() {

        let hud_offset = hud_node.height.resolve(window.height(), Vec2::ONE);
        
        // why "size/8" is necessary -> sprite issue?
        let eighth_size = ENEMY_SIZE / 8.0;
        let x_min = (window.width()/-2.0) + eighth_size;
        let x_max = (window.width()/2.0) - eighth_size;
        let y_min = (window.height()/-2.0) + eighth_size;
        // prevent player from moving "under" ui
        let y_max = (window.height()/2.0) - eighth_size - hud_offset.unwrap();
        
        for mut transform in enemy_query.iter_mut() {
            let mut translation = transform.translation;
            // constraint for enemy x and y pos
            translation.x = translation.x.clamp(x_min, x_max);
            translation.y = translation.y.clamp(y_min, y_max);
            // set new position
            transform.translation = translation;
        }  
    }
}

// tick every time.delta()
pub fn tick_spawn_enemy_timer(mut enemy_timer: ResMut<EnemyTimer>, time: Res<Time>) {
    enemy_timer.timer.tick(time.delta());
}

// pub fn despawn_enemies_over_time(
//     mut commands: Commands,
//     despawn_timer: Res<EnemyTimer>,
//     mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
// ) {
//     let mut enemy_entities: bevy::ecs::query::QueryIter<'_, '_, (Entity, &Transform), With<Enemy>> = enemy_query.iter_mut();
//     if let Some(enemy_entity) = enemy_entities.next() {
//         if despawn_timer.timer.finished() {
//             commands.entity(enemy_entity.0).despawn();
//         }
//     }
// }

pub fn despawn_enemies(
    mut commands: Commands,
    // mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    // let mut enemy_entities: bevy::ecs::query::QueryIter<'_, '_, (Entity, &Transform), With<Enemy>> = enemy_query.iter_mut();
    // if let Some(enemy_entity) = enemy_entities.next() {
    //     commands.entity(enemy_entity.0).despawn();
    // }
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    spawn_timer: Res<EnemyTimer>,
    hud_query: Query<&Node, With<Hud>>,
) {
    // println!("a");
    if spawn_timer.timer.finished() {
    // println!("b");
        
        if let (Ok(window), Ok(hud_node)) = (window_query.single(), hud_query.single()) {
    // println!("c");

            let hud_offset = hud_node.height.resolve(window.height(), Vec2::ONE);
            // spawn_pos: [-1..1] * win.y/2 - hud.y  
            let y = (rand::rng().random_range(-1.0..1.0) * window.height() / 2.) - hud_offset.unwrap();
            let x = rand::rng().random_range(-1.0..1.0) * window.width() / 2.;
            
            commands.spawn((
                Sprite::from_image(asset_server.load("sprites/tile_0005.png")),
                Transform::from_xyz(x, y, 0.0),
                // Transform::from_translation(vec),
                Enemy {
                    direction: Vec3::new(
                        (rand::rng().random_range(0..2) * 2 - 1) as f32,
                        (rand::rng().random_range(0..2) * 2 - 1) as f32,
                        0.
                    ).normalize_or_zero(),
                },
            ));
        }
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);

        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}
