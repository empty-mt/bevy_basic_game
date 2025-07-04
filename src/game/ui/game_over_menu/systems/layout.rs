use bevy::prelude::*;

use crate::game::ui::game_over_menu::styles::*;
use crate::game::ui::game_over_menu::components::*;
use crate::game::ui::main_menu::styles::*;
use crate::global_systems::ui_create_rounded_rect_button_node;


pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let main_menu_entity: Entity =
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(main_menu_entity) = game_over_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}
pub fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    // main menu
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let main_menu_entity = commands.spawn(
        (
            // Style lives now in Node
            Node { 
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),
                ..default() },
            GameOverMenu {})
    )
    .insert(BackgroundColor(UI_GAME_OVER_MENU_BG_COL))
    .id();
    
    //
    // children nodes
    //

    //
    // button resume
    //
    commands.spawn((
        RestartButton,
        ui_create_rounded_rect_button_node(),
        ui_get_rounded_rect_param(),
        // have to set the interaction manually, cause there is no default set?
        Interaction::default(),
        ))
        .insert(BackgroundColor(UI_RESTART_BUTTON_BG_COL))
        .insert(ChildOf(main_menu_entity))
        .with_children(
            |parent| {
                parent.spawn((
                    TextFont {
                        font: font.clone(),
                        font_size: 25.0,
                        ..Default::default()
                    },
                    Text::new("_ restart"),
                    TextColor::from(UI_FONT_COL),
                    ));
            });
    //
    // button quit app
    //
    commands.spawn((
        MainMenuButton,
        ui_create_rounded_rect_button_node(),
        ui_get_rounded_rect_param(),
        Interaction::default(),
        ))
        .insert(BackgroundColor(UI_RESTART_BUTTON_BG_COL))
        .insert(ChildOf(main_menu_entity))
        .with_children(
        |parent| {
            parent.spawn((
                TextFont {
                    font: font.clone(),
                    font_size: 25.0,
                    ..Default::default()
                },
                Text::new("_ main menu"),
                TextColor::from(UI_FONT_COL)
            ));
        });

    main_menu_entity
}