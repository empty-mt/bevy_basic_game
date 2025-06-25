use bevy::prelude::*;

#[derive(Component)]
pub struct Hud;
#[derive(Component)]
pub struct HudLeft;
#[derive(Component)]
pub struct HudRight;
#[derive(Component)]
pub struct HudMiddle;


#[derive(Component, Debug)]
pub struct ScoreDisplay;
#[derive(Component, Debug)]
pub struct ScoreText;
#[derive(Component, Debug)]
pub struct LevelText;
#[derive(Component, Debug)]
pub struct TimeText;

#[derive(Component)]
pub struct EnemyImage;

#[derive(Component)]
pub struct LevelImage;

#[derive(Component)]
pub struct TimeImage;