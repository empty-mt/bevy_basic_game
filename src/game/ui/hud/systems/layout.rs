use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;


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
    let image = asset_server.load("sprites/tile_0005.png");
    
    let hud_entity = commands.spawn(
        (
            //
            // ### hud parent
            //
            // component
            //
            Hud,
            //
            // main node
            //
            Node { 
                width: Val::Percent(100.0),
                height: Val::Percent(5.0),
                border: UiRect::all(Val::Px(2.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_content: AlignContent::FlexStart,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Start,
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(6.0)),
                row_gap: Val::Px(1.0),
                column_gap: Val::Px(1.0),
                ..default() 
            },
            //
            // additional style bundle
            //
            (
                BackgroundColor(HUD_BG_COL.into()),
                BorderColor(HUB_BORDER_COL.into()),
                BorderRadius::all(Val::Px(10.0))
            ),
            //
            // children nodes
            //
            children![

                // ### enemy image
                //
                // child node
                //
                (Node {
                    width: Val::Percent(2.0),
                    height: Val::Percent(95.0),
                    border: UiRect::all(Val::Px(1.0)),
                    display: Display::Flex,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                //
                // additional style bundle
                //
                (
                    BorderColor(HUB_BORDER_COL.into()),
                    BorderRadius::all(Val::Px(10.0))
                ),
                //
                // child of child
                //
                children![
                    ImageNode {
                        image: image,
                        ..default()
                    },
                    EnemyImage,
                ]),

                // ### score display
                //
                // component
                //
                ScoreDisplay,
                //
                // child node
                //
                (Node { 
                    width: Val::Percent(3.0),
                    height: Val::Percent(95.0),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    display: Display::Flex,
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    ..default() },
                    //
                    // additional style bundle
                    //
                    (
                        BorderColor(HUB_BORDER_COL.into()),
                        BorderRadius::all(Val::Px(10.0))
                    ),
                children![
                    TextFont {
                        font: font.clone(),
                        font_size: 25.0,
                        ..Default::default()
                    },
                    (
                        Text::new("XXX"),
                        TextColor::from(UI_FONT_COL)
                    )
                ]),
            ],
        )
    )
    .id();
    hud_entity
}