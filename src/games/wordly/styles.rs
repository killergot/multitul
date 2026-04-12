use crate::games::wordly::mark::Mark;
use iced::{
    Background, Border, Color, Shadow, Theme, Vector,
    widget::{button, container},
};

fn palette_by_mark(mark: Mark) -> (Color, Color) {
    match mark {
        Mark::Correct => (
            Color::from_rgb(0.42, 0.67, 0.39), // green fill
            Color::from_rgb(0.42, 0.67, 0.39), // green border
        ),
        Mark::Present => (
            Color::from_rgb(0.79, 0.71, 0.35), // yellow fill
            Color::from_rgb(0.79, 0.71, 0.35), // yellow border
        ),
        Mark::Absent => (
            Color::from_rgb(0.53, 0.53, 0.54), // gray fill
            Color::from_rgb(0.53, 0.53, 0.54), // gray border
        ),
        Mark::Cursor => (
            Color::from_rgb(0.12, 0.12, 0.10), // dark background
            Color::from_rgb(0.35, 0.35, 0.38), // active border
        ),
        Mark::Unknown => (
            Color::from_rgb(0.12, 0.12, 0.10), // empty cell
            Color::from_rgb(0.22, 0.22, 0.20), // neutral border
        ),
    }
}

fn mix(base: Color, accent: Color, amount: f32) -> Color {
    Color {
        r: base.r + (accent.r - base.r) * amount,
        g: base.g + (accent.g - base.g) * amount,
        b: base.b + (accent.b - base.b) * amount,
        a: base.a + (accent.a - base.a) * amount,
    }
}

pub fn marked_cell_style(mark: Mark) -> container::Style {
    let (background, border_color) = palette_by_mark(mark);

    match mark {
        Mark::Correct | Mark::Present | Mark::Absent | Mark::Cursor => container::Style {
            background: Some(background.into()),
            border: Border {
                width: 2.0,
                color: border_color,
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        Mark::Unknown => container::Style {
            border: Border {
                width: 2.0,
                color: border_color,
                radius: 6.0.into(),
            },
            ..Default::default()
        },
    }
}

pub fn keyboard_button_style(_theme: &Theme, status: button::Status, mark: Mark) -> button::Style {
    let (base_background, border_color) = palette_by_mark(mark);
    let background = if mark == Mark::Unknown {
        Color::from_rgb(0.14, 0.14, 0.16)
    } else {
        base_background
    };

    let mut style = button::Style {
        background: Some(Background::Color(background)),
        text_color: Color::WHITE,
        border: Border {
            width: 2.0,
            color: border_color,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.15),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 0.0,
        },
        snap: false,
    };

    match status {
        button::Status::Hovered => {
            style.border.width = 3.0;
            style.border.color = mix(border_color, Color::WHITE, 0.35);
            style.shadow.offset = Vector::new(0.0, 2.0);
            style.shadow.color = border_color.scale_alpha(0.35);
            style.background = Some(Background::Color(mix(background, border_color, 0.3)));
        }
        button::Status::Pressed => {
            style.border.width = 3.0;
            style.border.color = mix(border_color, Color::WHITE, 0.5);
            style.shadow.offset = Vector::new(0.0, 0.0);
            style.shadow.color = Color::TRANSPARENT;
            style.background = Some(Background::Color(mix(background, Color::BLACK, 0.25)));
        }
        button::Status::Disabled => {
            style.text_color = Color::from_rgb(0.6, 0.6, 0.6);
            style.background = Some(Background::Color(background.scale_alpha(0.5)));
            style.border.color = border_color.scale_alpha(0.5);
        }
        button::Status::Active => {}
    }

    style
}
