use crate::games::wordly::mark::Mark;
use crate::utils::style::{self, ACCENT, RADIUS_SM, SURFACE_0, TEXT, mix};
use iced::{
    Background, Border, Color, Shadow, Theme, Vector,
    widget::{button, container},
};

fn palette_by_mark(mark: Mark) -> (Color, Color) {
    match mark {
        Mark::Correct => (
            Color::from_rgb(0.42, 0.67, 0.39),
            Color::from_rgb(0.42, 0.67, 0.39),
        ),
        Mark::Present => (
            Color::from_rgb(0.79, 0.71, 0.35),
            Color::from_rgb(0.79, 0.71, 0.35),
        ),
        Mark::Absent => (
            Color::from_rgb(0.53, 0.53, 0.54),
            Color::from_rgb(0.53, 0.53, 0.54),
        ),
        Mark::Cursor => (SURFACE_0, ACCENT.scale_alpha(0.85)),
        Mark::Unknown => (SURFACE_0, Color::from_rgb(0.22, 0.24, 0.28)),
    }
}

pub fn marked_cell_style(mark: Mark) -> container::Style {
    let (background, border_color) = palette_by_mark(mark);

    match mark {
        Mark::Correct | Mark::Present | Mark::Absent | Mark::Cursor => container::Style {
            background: Some(background.into()),
            text_color: Some(TEXT),
            border: Border {
                width: 2.0,
                color: border_color,
                radius: RADIUS_SM.into(),
            },
            ..Default::default()
        },
        Mark::Unknown => container::Style {
            background: Some(background.into()),
            border: Border {
                width: 2.0,
                color: border_color,
                radius: RADIUS_SM.into(),
            },
            ..Default::default()
        },
    }
}

pub fn keyboard_button_style(_theme: &Theme, status: button::Status, mark: Mark) -> button::Style {
    let (base_background, border_color) = palette_by_mark(mark);
    let background = if mark == Mark::Unknown {
        Color::from_rgb(0.110, 0.133, 0.165)
    } else {
        base_background
    };

    let mut style = button::Style {
        background: Some(Background::Color(background)),
        text_color: TEXT,
        border: Border {
            width: 1.0,
            color: border_color,
            radius: RADIUS_SM.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        snap: false,
    };

    match status {
        button::Status::Hovered => {
            style.border.width = 1.5;
            style.border.color = mix(border_color, ACCENT, 0.45);
            style.background = Some(Background::Color(mix(background, ACCENT, 0.18)));
        }
        button::Status::Pressed => {
            style.border.width = 1.5;
            style.border.color = ACCENT;
            style.background = Some(Background::Color(mix(background, Color::BLACK, 0.20)));
        }
        button::Status::Disabled => {
            style.text_color = TEXT.scale_alpha(0.55);
            style.background = Some(Background::Color(background.scale_alpha(0.5)));
            style.border.color = border_color.scale_alpha(0.5);
        }
        button::Status::Active => {}
    }

    style
}

pub fn menu_panel(theme: &Theme) -> container::Style {
    style::surface(theme)
}

pub fn primary_button(theme: &Theme, status: button::Status) -> button::Style {
    style::primary_button(theme, status)
}

pub fn ghost_button(theme: &Theme, status: button::Status) -> button::Style {
    style::ghost_button(theme, status)
}

