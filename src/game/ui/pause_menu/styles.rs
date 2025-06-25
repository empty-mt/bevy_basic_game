use  bevy::{color::palettes::css::{LIGHT_BLUE, LIGHT_GREY}, prelude::*};

// rgba for transparency
pub const UI_PAUSE_MENU_BG_COL: Color = Color::linear_rgba(0.4, 0.8, 0.4, 0.25);
pub const UI_RESUME_BUTTON_BG_COL: Color = Color::linear_rgb(0.15, 0.6, 0.15);
pub const UI_QUIT_BUTTON_BG_COL: Color = Color::linear_rgb(0.15, 0.05, 0.0);
pub const UI_FONT_COL: Color = Color::linear_rgb(0.2, 0.45, 0.45);
pub const UI_HOVERED_BUTTON: Srgba = LIGHT_GREY;
pub const UI_PRESSED_BUTTON: Srgba = LIGHT_BLUE;