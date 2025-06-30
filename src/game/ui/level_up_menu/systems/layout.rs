use bevy::prelude::*;

use crate::game::ui::level_up_menu::components::*;
use crate::game::ui::level_up_menu::styles::*;
use crate::game::ui::main_menu::styles::*;
use crate::global_systems::ui_create_rounded_rect_button_node;

pub fn spawn_level_up_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let main_menu_entity: Entity =
    build_level_up_menu(&mut commands, &asset_server);
}

pub fn despawn_level_up_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<LevelUpMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}
pub fn build_level_up_menu(
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
                // make space for hud
                bottom: Val::Percent(-5.0),
                ..default() 
            },
            LevelUpMenu {},
        )
    )
    .insert(BackgroundColor(UI_LEVEL_UP_MENU_BG_COL))
    .id();
    
    //
    // children nodes
    //

    //
    // make fn button_gen(), if more buttons are needed
    //

    // text
    commands.spawn(
        (TextFont {
            font: font.clone(),
            font_size: 25.0,
            ..Default::default()
        },
        Text::new("! level up"),
        TextColor::from(UI_FONT_COL),
        ))
    .insert(ChildOf(main_menu_entity))
    ;

    // button continue
    commands.spawn((
        ContinueButton,
        ui_create_rounded_rect_button_node(),
        ui_get_rounded_rect_param(),
        // have to set the interaction manually, cause there is no default set?
        Interaction::default(),
        ))
        .insert(BackgroundColor(UI_CONTINUE_BUTTON_BG_COL))
        .insert(ChildOf(main_menu_entity))
        .with_children(
            |parent| {
                parent.spawn((TextFont {
                        font: font.clone(),
                        font_size: 25.0,
                        ..Default::default()
                    },
                    Text::new("_ continue"),
                    TextColor::from(UI_FONT_COL),
                    )
                );
            });
    // button upgrade
    commands.spawn((
        UpgradeButton,
        ui_create_rounded_rect_button_node(),
        ui_get_rounded_rect_param(),
        Interaction::default(),
        ))
        .insert(BackgroundColor(UI_CONTINUE_BUTTON_BG_COL))
        .insert(ChildOf(main_menu_entity))
        .with_children(
            |parent| {
                parent.spawn((TextFont {
                    font: font.clone(),
                    font_size: 25.0,
                    ..Default::default()
                },
                Text::new("_ upgrade"),
                TextColor::from(UI_FONT_COL)
            )
        );
    });
    
    // button main menu
    commands.spawn((
        MainMenuButton,
        ui_create_rounded_rect_button_node(),
        ui_get_rounded_rect_param(),
        Interaction::default(),
        ))
        .insert(BackgroundColor(UI_QUIT_BUTTON_BG_COL))
        .insert(ChildOf(main_menu_entity))
        .with_children(
        |parent| {
            parent.spawn((TextFont {
                    font: font.clone(),
                    font_size: 25.0,
                    ..Default::default()
                },
                Text::new("_ main menu"),
                TextColor::from(UI_FONT_COL)
            )
        );
    });
        main_menu_entity
}