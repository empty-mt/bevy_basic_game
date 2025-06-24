use bevy::prelude::*;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let main_menu_entity: Entity =
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}
pub fn build_main_menu(
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
            MainMenu {})
    )
    .insert(BackgroundColor(UI_MAIN_MENU_BG_COL))
    .id();
    
    //
    // children nodes
    //

    // gameinfo widget
    //
    // have to figure out, how to spawn meshes and gizmos or other elements
    //
    // ****** not finished ******
    //
    commands.spawn((
        Node {
            width: Val::Px(420.0),
            height: Val::Px(120.0),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        GameInfoWidget{}
        )
    )
    .insert(ChildOf(main_menu_entity))
    .insert(Visibility::default())
    .with_children(
        |parent| {
            // ### image part
            //
            // use UiImage or plain Node here

            // ### text part
            parent.spawn((
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..Default::default()
                },
                Text::new("_ Basic gamE"),
                TextColor::from(UI_FONT_COL),
            ));

            // ## image 2 part 
            parent.spawn(
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    ..default()
                }
            )
            .insert(BackgroundColor(UI_PLAY_BUTTON_BG_COL))
            ;
        }
    );
    //
    // ****** not finished ******
    //
    
    //
    // button play
    //
    commands.spawn((
        ui_create_basic_button_node(),
        PlayButton,
        // have to set the interaction manually, cause there is no default set?
        Interaction::default(),
        ))
        .insert(BackgroundColor(UI_PLAY_BUTTON_BG_COL))
        .insert(ChildOf(main_menu_entity))
        // debug:
        //
        // .observe(|trigger: Trigger<Pointer<Click>>| {
        //     info!("start");
        // })
        .with_children(
            |parent| {
                parent.spawn((
                    TextFont {
                        font: font.clone(),
                        font_size: 25.0,
                        ..Default::default()
                    },
                    Text::new("_ play"),
                    TextColor::from(UI_FONT_COL),
                    ));
            });

    // button quit app
    commands.spawn((
        ui_create_basic_button_node(),
        QuitButton,
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
                Text::new("_ quit"),
                TextColor::from(UI_FONT_COL)
            ));
        });

    main_menu_entity
}