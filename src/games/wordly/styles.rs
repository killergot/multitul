use iced::{Background, Border, Color, Shadow, Theme, Vector, widget::{button, container, text}};
use crate::games::wordly::mark::Mark;

pub const PAPER: Color = Color::from_rgb(0.91, 0.84, 0.70);
pub const PAPER_DEEP: Color = Color::from_rgb(0.84, 0.75, 0.60);
pub const PAPER_LIGHT: Color = Color::from_rgb(0.96, 0.91, 0.79);
pub const PAPER_EDGE: Color = Color::from_rgb(0.53, 0.39, 0.23);
pub const PAPER_STAIN: Color = Color::from_rgb(0.72, 0.58, 0.37);
pub const INK: Color = Color::from_rgb(0.22, 0.16, 0.08);
pub const INK_SOFT: Color = Color::from_rgb(0.38, 0.28, 0.16);
pub const DESK: Color = Color::from_rgb(0.49, 0.35, 0.22);
pub const DESK_DARK: Color = Color::from_rgb(0.29, 0.19, 0.10);

fn palette_by_mark(mark: Mark) -> (Color, Color) {
    match mark {
        Mark::Correct => (Color::from_rgb(0.61, 0.74, 0.44), Color::from_rgb(0.29, 0.38, 0.16)),
        Mark::Present => (Color::from_rgb(0.82, 0.67, 0.34), Color::from_rgb(0.50, 0.33, 0.12)),
        Mark::Absent => (Color::from_rgb(0.59, 0.50, 0.39), Color::from_rgb(0.32, 0.24, 0.15)),
        Mark::Cursor => (Color::from_rgb(0.95, 0.88, 0.74), Color::from_rgb(0.46, 0.28, 0.14)),
        Mark::Unknown => (mix(PAPER, Color::WHITE, 0.08), Color::from_rgb(0.57, 0.42, 0.26)),
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

    let shadow = match mark {
        Mark::Correct | Mark::Present => Shadow {
            color: border_color.scale_alpha(0.30),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 0.0,
        },
        Mark::Cursor => Shadow {
            color: Color::from_rgb(0.42, 0.24, 0.11).scale_alpha(0.22),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 0.0,
        },
        _ => Shadow {
            color: Color::BLACK.scale_alpha(0.12),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 0.0,
        },
    };

    container::Style {
        background: Some(background.into()),
        border: Border {
            width: if mark == Mark::Unknown { 1.5 } else { 2.0 },
            color: border_color,
            radius: 12.0.into(),
        },
        shadow,
        text_color: None,
        snap: false,
    }
}

pub fn keyboard_button_style(_theme: &Theme, status: button::Status, mark: Mark) -> button::Style {
    let (base_background, border_color) = palette_by_mark(mark);
    let background = if mark == Mark::Unknown {
        mix(PAPER, PAPER_DEEP, 0.18)
    } else {
        base_background
    };

    let mut style = button::Style {
        background: Some(Background::Color(background)),
        text_color: INK,
        border: Border {
            width: 1.5,
            color: border_color,
            radius: 10.0.into(),
        },
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.14),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 0.0,
        },
        snap: false,
    };

    match status {
        button::Status::Hovered => {
            style.border.width = 2.5;
            style.border.color = mix(border_color, Color::WHITE, 0.25);
            style.shadow.offset = Vector::new(0.0, 3.0);
            style.shadow.color = border_color.scale_alpha(0.35);
            style.background = Some(Background::Color(mix(background, PAPER, 0.22)));
        }
        button::Status::Pressed => {
            style.border.width = 2.0;
            style.border.color = mix(border_color, Color::WHITE, 0.18);
            style.shadow.offset = Vector::new(0.0, 1.0);
            style.shadow.color = Color::TRANSPARENT;
            style.background = Some(Background::Color(mix(background, PAPER_EDGE, 0.18)));
        }
        button::Status::Disabled => {
            style.text_color = INK_SOFT.scale_alpha(0.6);
            style.background = Some(Background::Color(background.scale_alpha(0.5)));
            style.border.color = border_color.scale_alpha(0.5);
        }
        button::Status::Active => {}
    }

    style
}

pub fn paper_panel_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(PAPER, PAPER_LIGHT, 0.18))),
        border: Border {
            width: 2.0,
            color: PAPER_EDGE,
            radius: 24.0.into(),
        },
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.20),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 0.0,
        },
        text_color: None,
        snap: false,
    }
}

pub fn paper_sheet_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(PAPER_LIGHT, PAPER, 0.55))),
        border: Border {
            width: 1.0,
            color: mix(PAPER_EDGE, Color::WHITE, 0.12),
            radius: 30.0.into(),
        },
        shadow: Shadow {
            color: DESK_DARK.scale_alpha(0.25),
            offset: Vector::new(0.0, 14.0),
            blur_radius: 0.0,
        },
        text_color: None,
        snap: false,
    }
}

pub fn desk_background_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(DESK)),
        border: Border {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: 0.0.into(),
        },
        shadow: Shadow {
            color: DESK_DARK.scale_alpha(0.45),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        text_color: None,
        snap: false,
    }
}

pub fn paper_crease_style(strength: f32) -> container::Style {
    let shade = mix(PAPER_DEEP, PAPER_EDGE, strength.clamp(0.0, 1.0) * 0.35);
    container::Style {
        background: Some(Background::Color(shade.scale_alpha(0.55))),
        border: Border {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: 999.0.into(),
        },
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.04 + strength * 0.10),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 0.0,
        },
        text_color: None,
        snap: false,
    }
}

pub fn paper_highlight_style(strength: f32) -> container::Style {
    let tint = mix(PAPER_LIGHT, Color::WHITE, 0.25);
    container::Style {
        background: Some(Background::Color(tint.scale_alpha(0.24 + strength.clamp(0.0, 1.0) * 0.18))),
        border: Border {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: 999.0.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        text_color: None,
        snap: false,
    }
}

pub fn paper_stain_style(alpha: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(PAPER_STAIN.scale_alpha(alpha.clamp(0.0, 1.0)))),
        border: Border {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: 999.0.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        text_color: None,
        snap: false,
    }
}

pub fn torn_edge_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(PAPER_DEEP, PAPER_EDGE, 0.14))),
        border: Border {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: 999.0.into(),
        },
        shadow: Shadow {
            color: DESK_DARK.scale_alpha(0.12),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 0.0,
        },
        text_color: None,
        snap: false,
    }
}

pub fn menu_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::Style {
        background: Some(Background::Color(mix(PAPER_DEEP, PAPER, 0.35))),
        text_color: INK,
        border: Border {
            width: 1.5,
            color: PAPER_EDGE,
            radius: 14.0.into(),
        },
        shadow: Shadow {
            color: DESK_DARK.scale_alpha(0.18),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 0.0,
        },
        snap: false,
    };

    match status {
        button::Status::Hovered => {
            style.background = Some(Background::Color(mix(PAPER, Color::WHITE, 0.12)));
            style.border.color = mix(PAPER_EDGE, INK, 0.15);
            style.shadow.offset = Vector::new(0.0, 5.0);
        }
        button::Status::Pressed => {
            style.background = Some(Background::Color(mix(PAPER_DEEP, PAPER_EDGE, 0.16)));
            style.shadow.offset = Vector::new(0.0, 1.0);
        }
        button::Status::Disabled => {
            style.text_color = INK_SOFT.scale_alpha(0.55);
            style.background = Some(Background::Color(PAPER_DEEP.scale_alpha(0.5)));
            style.border.color = PAPER_EDGE.scale_alpha(0.45);
        }
        button::Status::Active => {}
    }

    style
}

pub fn title_style(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(INK),
    }
}

pub fn body_text_style(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(INK_SOFT),
    }
}
