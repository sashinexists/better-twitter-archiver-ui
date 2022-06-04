use iced::Color;
pub const MAIN_TEXT_COLOR: Color = colors::CULTURED_WHITE;
pub const CONTENT_BG_COLOR: Color = colors::RAISIN_BLACK;
pub const CONTENT_HIGHLIGHT_BG_COLOR: Color = colors::RAISIN_BLACK_LIGHT;
pub const MAIN_BG_COLOR: Color = colors::RICH_BLACK;
mod colors {
    use iced::Color;
    pub const RAISIN_BLACK: Color = Color::from_rgb(0.12, 0.11, 0.12);
    pub const RAISIN_BLACK_LIGHT: Color = Color::from_rgb(0.14, 0.18, 0.16);
    pub const RICH_BLACK: Color = Color::from_rgb(0.047, 0.035, 0.0431);
    pub const CULTURED_WHITE: Color = Color::from_rgb(0.96, 0.956, 0.96);
}
