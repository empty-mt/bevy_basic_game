use bevy::prelude::*;
use bevy::text::ComputedTextBlock;

use crate::game::ui::hud::styles::*;
use crate::game::ui::hud::components::*;


pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let hud_entity: Entity =
    build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<Hud>>,
) {
    if let Ok(hud_entity) = hud_query.single() {
        commands.entity(hud_entity).despawn();
    }
}
pub fn build_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let image_enemy = asset_server.load("sprites/target_round_b.png");
    let image_time = asset_server.load("sprites/busy_hourglass_outline_detail.png");
    let image_level = asset_server.load("sprites/door.png");
    
    let hud = commands.spawn(
        (
            Hud,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0), 
                height: Val::Percent(5.0),
                ..default()
            },
            children![
                (
                    HudLeft,
                    ui_get_hud_part_style_node(),
                    (
                        BackgroundColor(HUD_BG_COL.into()),
                    ),
                    children![
                        // enemy image
                        (
                        ui_get_image_style_node(),
                        (
                            BorderColor(HUD_BORDER_COL.into()),
                            BorderRadius::all(Val::Px(10.0))
                        ),
                        children![
                            (
                            EnemyImage,
                            ImageNode {
                                image: image_enemy,
                                ..default()
                            },
                            )
                        ]),

                        // score display
                        ScoreDisplay,
                        (
                        ui_get_text_style_node(),
                        (
                            BorderColor(HUD_BORDER_COL.into()),
                            BorderRadius::all(Val::Px(10.0)),
                            TextLayout::default(),
                            ComputedTextBlock::default(),
                        ),
                        children![
                            ( // <--- ! fml .../(*.*/)
                            ScoreText,
                            ui_get_text_style("0".to_string(), UI_FONT_COL, &font),
                            ) // <--- !
                            ]
                        ),
                    ],
                ),
                (
                    HudMiddle,
                    ui_get_hud_part_style_node(),
                    (
                        BackgroundColor(HUD_BG_COL.into()),
                    ),
                    children![
                        // level image
                        (
                        ui_get_image_style_node(),
                        (
                            BorderColor(HUD_BORDER_COL.into()),
                            BorderRadius::all(Val::Px(10.0))
                        ),
                        children![
                            (
                            LevelImage,
                            ImageNode {
                                image: image_level,
                                ..default()
                            },
                            )
                        ]),
                        // level display
                        (
                        ui_get_text_style_node(),
                        (
                            BorderColor(HUD_BORDER_COL.into()),
                            BorderRadius::all(Val::Px(10.0)),
                            TextLayout::default(),
                            ComputedTextBlock::default(),
                        ),
                        children![
                            ( // <--- ! fml .../(*.*/)
                            LevelText,
                            ui_get_text_style("1".to_string(), UI_FONT_COL, &font),
                            ) // <--- !
                            ]
                        ),
                    ],
                ),
                (
                    HudRight,
                    ui_get_hud_part_style_node(),
                    (
                        BackgroundColor(HUD_BG_COL.into()),
                    ),
                    children![
                        // time image
                        (
                        ui_get_image_style_node(),
                        (
                            BorderColor(HUD_BORDER_COL.into()),
                            BorderRadius::all(Val::Px(10.0))
                        ),
                        children![
                            (
                            TimeImage,
                            ImageNode {
                                image: image_time,
                                ..default()
                            },
                            )
                        ]),
                        // time display
                        (
                        ui_get_text_style_node(),
                        (
                            BorderColor(HUD_BORDER_COL.into()),
                            BorderRadius::all(Val::Px(10.0)),
                            TextLayout::default(),
                            ComputedTextBlock::default(),
                        ),
                        children![
                            ( // <--- ! fml .../(*.*/)
                            TimeText,
                            ui_get_text_style("0".to_string(), UI_FONT_COL, &font),
                            ) // <--- !
                            ]
                        ),
                    ],
                )
            ],
        )
    )
    .id();
    hud
}