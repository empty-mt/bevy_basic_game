// using 0.10 guide
// https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=2

use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub const PLAYER_SIZE: f32 = 64.0;      // player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SIZE: f32 = 64.0;       // nmy sprite size
pub const ENEMY_SPEED: f32 = 100.0;
pub const NUM_ENEMIES: i16 = 16;
pub const GLOBAL_VOLUME: f32 = 0.1;     // [0.0 - 1.0]
pub const ENEMY_DESPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct EnemyTimer {
    pub timer: Timer,
}

impl Default for EnemyTimer {
    fn default() -> EnemyTimer {
        EnemyTimer {
            timer: Timer::from_seconds(ENEMY_DESPAWN_TIME, TimerMode::Repeating)
        }
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: i32,
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0, }
    }
}
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

fn main() {
    let mut app = App::new();
    
    app
    .add_plugins(DefaultPlugins)
    .add_plugins(CustomPlugin)
    // global volume
    .insert_resource(GlobalVolume::new(Volume::Decibels(GLOBAL_VOLUME)))
    .init_resource::<Score>() 
    .init_resource::<EnemyTimer>() 
    
    .run();
}

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        app    
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, max_window)
        // .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, change_global_volume)
        // .add_systems(Update, spawn_enemies)
        
        // .add_systems(Update, despawn_enemies_over_time)
        .add_systems(Update, tick_despawn_enemy_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, update_score)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, update_enemy_movement);
    }
}

pub fn max_window(windows: Query<&mut Window>) {
    for mut window in windows {
        window.set_maximized(true);
    }
}

fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
    volume.volume = Volume::Linear(GLOBAL_VOLUME);
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

pub fn spawn_enemies(
    // keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    // if keyboard_input.just_pressed(KeyCode::KeyK) {
        for _ in 0..NUM_ENEMIES {
            let y = rand::rng().random_range(-1.0..1.0) * window.height() / 2.;
            let x = rand::rng().random_range(-1.0..1.0) * window.width() / 2.;
            // !! max_window does not upscale window.size()
            //
            // let vec = Vec3::new(x, y, 0.0);
            // println!("{} {} == {:?} || {:?}",x,y,vec, window.size());

            commands.spawn((
                Sprite::from_image(asset_server.load("sprites/tile_0005.png")),
                // Sprite {
                //     image: asset_server.load("sprites/tile_0005.png"), 
                //     // image_mode: Auto,
                //     // rect: Some(Rect::from_center_size(Vec2::new(x, y), Vec2::ONE)), 
                //     ..default() },
                Transform::from_xyz(x, y, 0.0),
                // Transform::from_translation(vec),
                Enemy {
                    direction: Vec3::new(
                        // rand::rng().random_range(-1..1) as f32,
                        // rand::rng().random_range(-1..1) as f32,
                        (rand::rng().random_range(0..2) * 2 - 1) as f32,
                        (rand::rng().random_range(0..2) * 2 - 1) as f32,
                        0.
                    ).normalize_or_zero(),
                },
            ));
        }
    // }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let _window = window_query.single().unwrap();

    commands.spawn(Camera2d{});
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

// enemy hits wall
pub fn update_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // Single instead of Query 
    // mut music_controller: Single<&mut AudioSink>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,

) {
    let window = window_query.single().unwrap();
    // why "size/8" is necessary -> sprite issue?
    let eighth_size = ENEMY_SIZE / 8.0;
    let x_min = (window.width()/-2.0) + eighth_size;
    let x_max = (window.width()/2.0) - eighth_size;
    let y_min = (window.height()/-2.0) + eighth_size;
    let y_max = (window.height()/2.0) - eighth_size;

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
//
// if enemy movement is buggy in future then mb need: confine_enemy_movement()
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

        // contraint for player x and y pos
        translation.x = translation.x.clamp(x_min, x_max);
        translation.y = translation.y.clamp(y_min, y_max);
        // set new position
        player_transform.translation = translation;
    }
    
}

// check if pixel of player and nmy are overlapping
pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    // mut enemy_query: Query<&Transform, With<Enemy>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        // for enemy_transform in enemy_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {

            let dist = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 8.;
            let enemy_radius = ENEMY_SIZE / 8.;

            // despawn enemy
            if dist < player_radius + enemy_radius {
                let sound = asset_server.load("audio/pluck_002.ogg");
                
                commands.spawn(AudioPlayer::<AudioSource>(sound));
                // commands.entity(player_entity).despawn();
                commands.entity(enemy_entity).despawn();
                score.value +=1;
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("{:?}", score.value);
    }
}

// tick every time.delta()
pub fn tick_despawn_enemy_timer(mut enemy_timer: ResMut<EnemyTimer>, time: Res<Time>) {
    enemy_timer.timer.tick(time.delta());
}

pub fn despawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    despawn_timer: Res<EnemyTimer>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let mut enemy_entities = enemy_query.iter_mut();
    if let Some(enemy_entity) = enemy_entities.next() {
        if despawn_timer.timer.finished() {
            commands.entity(enemy_entity.0).despawn();
        }
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    despawn_timer: Res<EnemyTimer>,
) {
    if despawn_timer.timer.finished() {
            let window = window_query.single().unwrap();
            let y = rand::rng().random_range(-1.0..1.0) * window.height() / 2.;
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
            )
        );
    }
}