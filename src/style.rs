use crate::theme;
use iced::pure::widget::{button, container};

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

pub struct NavButtonInactive;

impl button::StyleSheet for NavButtonInactive {
    fn active(&self) -> button::Style {
        button::Style {
            background: theme::CONTENT_BG_COLOR.into(),
            text_color: theme::CONTENT_BG_COLOR.into(),
            border_radius: 10.0,
            ..button::Style::default()
        }
    }
}
