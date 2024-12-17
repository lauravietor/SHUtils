use iced::theme::{Palette, Theme};
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
