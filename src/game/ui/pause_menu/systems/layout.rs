use bevy::prelude::*;

use crate::global_systems::ui_create_basic_button_node;
use crate::game::ui::pause_menu::components::*;
use crate::game::ui::pause_menu::styles::*;

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let main_menu_entity: Entity =
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}
pub fn build_pause_menu(
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
            PauseMenu {},
        )
    )
    .insert(BackgroundColor(UI_PAUSE_MENU_BG_COL))
    .id();
    
    //
    // children nodes
    //

    // button resume
    //
    // make fn button_gen(), if more buttons are needed
    //
    commands.spawn((
        ui_create_basic_button_node(),
        ResumeButton,
        // have to set the interaction manually, cause there is no default set?
        Interaction::default(),
        ))
        .insert(BackgroundColor(UI_RESUME_BUTTON_BG_COL))
        .insert(ChildOf(main_menu_entity))
        .with_children(
            |parent| {
                parent.spawn((
                    TextFont {
                        font: font.clone(),
                        font_size: 25.0,
                        ..Default::default()
                    },
                    Text::new("_ resume"),
                    TextColor::from(UI_FONT_COL),
                    ));
            });

    // button quit app
    commands.spawn((
        ui_create_basic_button_node(),
        MainMenuButton,
        Interaction::default(),
        ))
        .insert(BackgroundColor(UI_QUIT_BUTTON_BG_COL))
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