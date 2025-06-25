use  bevy::{color::palettes::css::{LIGHT_BLUE, LIGHT_GREY}, prelude::*};
use crate::game::ui::hud::styles::*;

pub const UI_MAIN_MENU_BG_COL: Color = Color::linear_rgb(0.4, 0.8, 0.4);
pub const UI_PLAY_BUTTON_BG_COL: Color = Color::linear_rgb(0.15, 0.6, 0.15);
pub const UI_QUIT_BUTTON_BG_COL: Color = Color::linear_rgb(0.15, 0.05, 0.0);
// pub const UI_FONT_COL: Color = Color::linear_rgb(0.2, 0.45, 0.45);
pub const UI_HOVERED_BUTTON: Srgba = LIGHT_GREY;
pub const UI_PRESSED_BUTTON: Srgba = LIGHT_BLUE;

pub fn ui_get_rounded_rect_param() -> (bevy::prelude::BorderColor, bevy::prelude::BackgroundColor, bevy::prelude::BorderRadius) {
    (
        BorderColor(HUD_BORDER_COL.into()),
        BackgroundColor(HUD_BG_COL.into()),
        BorderRadius::all(Val::Px(10.0)),
)

}
pub fn ui_create_basic_button_node() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    }
}