use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    // let main_menu_entity: Entity =
    build_main_menu(&mut commands, meshes, &asset_server, materials, gizmo_assets);
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
    meshes: ResMut<Assets<Mesh>>,
    asset_server: &Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
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
            // image part
            let mut gizmo = GizmoAsset::default();

            gizmo
            .primitive_2d( &Circle::new(5.0), 
                Isometry2d::default(), 
                RED_300
            )
            .resolution(64);

            parent.spawn( Gizmo {
                handle: gizmo_assets.add(gizmo),
                        line_config: GizmoLineConfig {
                            width: 4.,
                            ..default()
                        },
                ..default()
            });
            // parent.spawn(
            //     Node {
            //         width: Val::Px(100.0),
            //         height: Val::Px(100.0),
            //         ..default()
            //     }
            // )
            // .insert(BackgroundColor(UI_PLAY_BUTTON_BG_COL))
            // .insert(BorderRadius {
            //     top_left: Val::Px(100.0),
            //     top_right: Val::Px(100.0),
            //     bottom_left: Val::Px(100.0),
            //     bottom_right: Val::Px(100.0),
            // })
            
        // Sprite {
        //         image: asset_server.load("sprites/tile_0005.png"),
        //         color: Color::BLACK, 
        //         ..default() },

            // text part
            parent.spawn((
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..Default::default()
                },
                Text::new("_ Basic gamE"),
                TextColor::from(UI_FONT_COL),
            ));

            // image 2 part 
            parent.spawn(
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    ..default()
                }
            )
            .insert(BackgroundColor(UI_PLAY_BUTTON_BG_COL))
            ;

            // let shape = meshes.add(Circle::default());
            // parent.spawn((
            //     Mesh2d(shape),
            //     MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::BLACK))),
            // ));
        }
    );
    //
    // ****** not finished ******
    //
    
    // button play
    //
    // make fn button_gen(), if more buttons are needed
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