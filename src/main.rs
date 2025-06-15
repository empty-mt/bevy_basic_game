use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

pub const PLAYER_SPEED: f32 = 500.0;
//sprite size
pub const PLAYER_SIZE: f32 = 64.0; 
pub const NUM_ENEMIES: i8 = 4;

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
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        // .add_systems(Startup, spawn_enemies)
        
        .add_systems(Update, spawn_enemies)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    if keyboard_input.just_pressed(KeyCode::KeyK) {
        for _ in 0..NUM_ENEMIES {
            // let y = (window.height() - (random::<f32>() * window.height()))/4.;
            // let x = (window.width() - (random::<f32>() * window.width()))/4.;
            let y = rand::rng().random_range(-1.0..1.0) * window.height()/2.;
            let x = rand::rng().random_range(-1.0..1.0) * window.width()/2.;
            let vec = Vec3::new(x, y, 0.0);
            // println!("{} {} == {:?}",x,y,vec);
            // todo!();
            commands.spawn((
                Sprite::from_image(asset_server.load("sprites/tile_0005.png")),
                // Sprite {
                //     image: asset_server.load("sprites/tile_0005.png"), 
                //     // image_mode: Auto,
                //     // rect: Some(Rect::from_center_size(Vec2::new(x, y), Vec2::ONE)), 
                //     ..default() },
                Transform::from_xyz(x, y, 0.0),
                // Transform::from_translation(vec),
                Enemy {},
            ));
        }
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.single().unwrap();

    commands.spawn(Camera2d{});
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
        direction.normalize_or_zero();
        
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
        // still wondering why "size/8" is necessary -> sprite issue?
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