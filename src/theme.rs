use iced::theme::{Palette, Theme};
use iced::widget::{button, container};
use iced::Color;

pub fn make_theme(_state: &crate::State) -> Theme {
    Theme::custom(
        "SHUtils theme".into(),
        Palette {
            background: Color::from_rgb8(38, 44, 57),
            text: Color::from_rgb8(240, 240, 240),
            primary: Color::from_rgb8(53, 60, 75),
            success: Color::from_rgb8(21, 100, 35),
            danger: Color::from_rgb8(94, 16, 17),
        },
    )
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Color::from_rgb8(47, 54, 69).into()),
        border: iced::Border {
            radius: iced::border::Radius::from(16),
            ..iced::Border::default()
        },
        ..container::Style::default()
    }
}

pub fn side_view(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Color::from_rgb8(47, 54, 69).into()),
        ..container::Style::default()
    }
}

pub fn navbar(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Color::from_rgb8(53, 60, 75).into()),
        ..container::Style::default()
    }
}

pub fn counter_button(theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(89, 105, 139))),
        border: iced::Border {
            radius: iced::border::Radius::from(50),
            ..iced::Border::default()
        },
        text_color: theme.palette().text,
        ..button::Style::default()
    }
}
