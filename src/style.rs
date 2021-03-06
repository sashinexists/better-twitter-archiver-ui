use crate::theme;

use iced::{
    pure::{
        scrollable,
        widget::{button, container, scrollable, text_input},
    },
    TextInput,
};
pub struct App;

impl container::StyleSheet for App {
    fn style(&self) -> container::Style {
        container::Style {
            background: theme::MAIN_BG_COLOR.into(),
            text_color: theme::MAIN_TEXT_COLOR.into(),
            ..container::Style::default()
        }
    }
}

pub struct Tweet;

impl button::StyleSheet for Tweet {
    fn active(&self) -> button::Style {
        button::Style {
            background: theme::CONTENT_BG_COLOR.into(),
            text_color: theme::MAIN_TEXT_COLOR.into(),
            border_radius: 10.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: theme::MAIN_TEXT_COLOR.into(),
            background: theme::CONTENT_HIGHLIGHT_BG_COLOR.into(),
            ..self.active()
        }
    }
}

pub struct NavButton;

impl button::StyleSheet for NavButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: theme::BUTTON_BG_COLOR.into(),
            text_color: theme::BUTTON_TEXT_COLOR.into(),
            border_radius: 10.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: theme::MAIN_TEXT_COLOR.into(),
            background: theme::BUTTON_HOVER_BG_COLOR.into(),
            ..self.active()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            text_color: theme::BUTTON_TEXT_INACTIVE_COLOR.into(),
            background: theme::BUTTON_BG_COLOR.into(),
            ..self.active()
        }
    }
}

pub struct MoreTweetsButton;

impl button::StyleSheet for MoreTweetsButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: theme::BUTTON_BG_COLOR.into(),
            text_color: theme::BUTTON_TEXT_COLOR.into(),
            border_radius: 10.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: theme::MAIN_TEXT_COLOR.into(),
            background: theme::BUTTON_HOVER_BG_COLOR.into(),
            ..self.active()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            text_color: theme::BUTTON_TEXT_INACTIVE_COLOR.into(),
            background: theme::BUTTON_BG_COLOR.into(),
            ..self.active()
        }
    }
}

pub struct SearchBar;

impl text_input::StyleSheet for SearchBar {
    fn active(&self) -> iced::text_input::Style {
        text_input::Style {
            background: theme::BUTTON_BG_COLOR.into(),
            border_radius: 10.0,
            border_width: 0.0,
            border_color: theme::BUTTON_BG_COLOR.into(),
        }
    }

    fn focused(&self) -> iced::text_input::Style {
        text_input::Style {
            background: theme::BUTTON_TEXT_INACTIVE_COLOR.into(),
            border_radius: 10.0,
            border_width: 0.0,
            border_color: theme::BUTTON_TEXT_INACTIVE_COLOR.into(),
        }
    }

    fn placeholder_color(&self) -> iced::Color {
        theme::BUTTON_TEXT_INACTIVE_COLOR.into()
    }

    fn value_color(&self) -> iced::Color {
        theme::BUTTON_TEXT_COLOR.into()
    }

    fn selection_color(&self) -> iced::Color {
        theme::CONTENT_HIGHLIGHT_BG_COLOR.into()
    }
}
