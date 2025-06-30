use bevy::prelude::*;
use crate::game::ui::main_menu::components::*;
use crate::game::ui::main_menu::styles::*;
use crate::game::ui::hud::styles::*;
use crate::global_systems::*;

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
            MainMenu {},
            Node { 
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                // position_type: PositionType::Absolute,
                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),
                ..default()
            },
            // background main menu
            ImageNode {
                image: asset_server.load("sprites/alienYellow_square.png"),
                image_mode: NodeImageMode::Tiled { tile_x: true, tile_y: true, stretch_value: 1.0 },
                ..default()
            },
        )
    )
    .id();

    // children nodes
        
    // gameinfo widget
    commands.spawn((
        Node { 
            width: Val::Px(200.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            padding: UiRect::all(Val::Px(8.0)),           
            border: UiRect::all(Val::Px(4.0)),            
            
            ..default()
        },
        ui_get_rounded_rect_param(),

    ))
    .with_children(|parent| {
        // border
        for offset in BORDER_OFFSETS.iter() {
            parent.spawn(
        (Node {
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_self: JustifySelf::Center,

                    // left: Val::Px(offset.x+50.0),
                    // top: Val::Px(offset.y+20.0),

                    // left: Val::Percent(50.0),
                    // top: Val::Percent(50.0),
                    // margin: UiRect {
                    //     left: Val::Px(offset.x - 40.0 / 2.0),
                    //     top: Val::Px(offset.y - 40.0 / 2.0),
                    //     ..default()
                    // },

                    left: Val::Px(offset.x),
                    right: Val::Px(-offset.x),
                    top: Val::Px(offset.y),
                    bottom: Val::Px(-offset.y),

                    ..default()
                },
                // Transform {
                //     translation: Vec3::new((offset.x - 40.0 / 2.0), (offset.y - 40.0 / 2.0), 0.0),
                //     ..default()
                // },
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                Text::new("get em"),
                TextColor::from(HUD_BORDER_COL),
                // BorderColor(HUD_BORDER_COL.into()),
                // BackgroundColor(HUD_BG_COL.into()),
                // ..default()
            ));
        }
        // title
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,

                // left: Val::Px(0.0+50.0),
                // top: Val::Px(0.0+20.0),

                // left: Val::Percent(50.0),
                // top: Val::Percent(50.0),
                // margin: UiRect {
                //     left: Val::Px( - 40.0 / 2.0),
                //     top: Val::Px( - 40.0 / 2.0),
                //     ..default()
                // },
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            // Transform {
            //     translation: Vec3::new((-40.0 / 2.0), (-40.0 / 2.0), 0.0),
            //     ..default()
            // },
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..Default::default()
            },
            Text::new("get em"),
            TextColor::from(UI_FONT_COL),
        ));
    })
    .insert(ChildOf(main_menu_entity))
    ;
    
    // button play
    commands.spawn((
        PlayButton,
        ui_create_rounded_rect_button_node(),
        ui_get_rounded_rect_param(),
        // have to set the interaction manually, cause there is no default set?
        Interaction::default(),
        ))
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
        QuitButton,
        ui_create_rounded_rect_button_node(),
        ui_get_rounded_rect_param(),
        Interaction::default(),
        ))
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