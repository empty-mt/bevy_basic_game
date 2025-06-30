use bevy::{color::palettes::css::{LIGHT_GREY, WHITE}, prelude::*};

pub const HUD_BG_COL: Srgba = LIGHT_GREY;
// pub const HUD_BG_COL: Srgba = LIGHT_GREY;
pub const HUD_BORDER_COL: Srgba = WHITE;
pub const UI_FONT_COL: Color = Color::linear_rgb(0.2, 0.45, 0.45);

pub fn ui_get_text_style(text: String, text_color: Color, font: &Handle<Font>) -> (bevy::ui::widget::Text, TextFont, TextColor) {
    (
    Text::new(text),
    TextFont {
        font: font.clone(),
        font_size: 25.0,
        ..Default::default()
    },
    TextColor::from(text_color),
    )
}

pub fn ui_get_image_style_node() -> Node {
    Node {
        width: Val::Percent(5.0),
        height: Val::Percent(95.0),
        border: UiRect::all(Val::Px(2.0)),
        display: Display::Flex,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn ui_get_text_style_node() -> Node {
    Node { 
        width: Val::Percent(5.0),
        height: Val::Percent(95.0),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        display: Display::Flex,
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        ..default() 
    }
}

pub fn ui_get_hud_part_style_node() -> Node {
    Node { 
        width: Val::Percent(33.3),
        height: Val::Percent(100.0),
        border: UiRect::all(Val::Px(2.0)),
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        justify_items: JustifyItems::Start,
        // position_type: PositionType::Absolute,
        padding: UiRect::all(Val::Px(1.0)),
        row_gap: Val::Px(1.0),
        column_gap: Val::Px(1.0),
        ..default() 
    }
}
