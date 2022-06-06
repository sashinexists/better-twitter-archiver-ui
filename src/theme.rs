use iced::Color;

use self::colors::CULTURED_WHITE_TRANSPARENT;
pub const MAIN_TEXT_COLOR: Color = colors::CULTURED_WHITE;
pub const BUTTON_TEXT_COLOR: Color = MAIN_TEXT_COLOR;
pub const BUTTON_TEXT_INACTIVE_COLOR: Color = CULTURED_WHITE_TRANSPARENT;
pub const CONTENT_BG_COLOR: Color = colors::RAISIN_BLACK;
pub const BUTTON_BG_COLOR: Color = CONTENT_BG_COLOR;
pub const BUTTON_INACTIVE_BG_COLOR: Color = colors::RAISIN_BLACK_TRANSPARENT;
pub const CONTENT_HIGHLIGHT_BG_COLOR: Color = colors::RAISIN_BLACK_LIGHT;
pub const BUTTON_HOVER_BG_COLOR: Color = CONTENT_HIGHLIGHT_BG_COLOR;
pub const MAIN_BG_COLOR: Color = colors::RICH_BLACK;
mod colors {
    use iced::Color;
    pub const RAISIN_BLACK: Color = Color::from_rgb(0.12, 0.11, 0.12);
    pub const RAISIN_BLACK_TRANSPARENT: Color = Color::from_rgba(0.12, 0.11, 0.12, 0.05);
    pub const RAISIN_BLACK_LIGHT: Color = Color::from_rgb(0.14, 0.18, 0.16);
    pub const RICH_BLACK: Color = Color::from_rgb(0.047, 0.035, 0.0431);
    pub const CULTURED_WHITE: Color = Color::from_rgb(0.96, 0.956, 0.96);
    pub const CULTURED_WHITE_TRANSPARENT: Color = Color::from_rgba(0.96, 0.956, 0.96, 0.05);
}
