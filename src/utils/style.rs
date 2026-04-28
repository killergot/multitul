use iced::{
    Background, Border, Color, Font, Shadow, Theme, Vector,
    font::{Family, Stretch, Style, Weight},
    widget::{button, container, scrollable, text_input},
};

pub const SURFACE_0: Color = Color::from_rgb(0.043, 0.055, 0.071);
pub const SURFACE_1: Color = Color::from_rgb(0.075, 0.094, 0.125);
pub const SURFACE_2: Color = Color::from_rgb(0.110, 0.133, 0.165);
pub const SURFACE_3: Color = Color::from_rgb(0.150, 0.180, 0.220);
pub const DIVIDER: Color = Color::from_rgb(0.180, 0.215, 0.255);
pub const ACCENT: Color = Color::from_rgb(0.910, 0.627, 0.290);
pub const ACCENT_DIM: Color = Color::from_rgb(0.722, 0.471, 0.227);
pub const TEAL: Color = Color::from_rgb(0.247, 0.631, 0.600);
pub const TEXT: Color = Color::from_rgb(0.910, 0.890, 0.835);
pub const TEXT_DIM: Color = Color::from_rgb(0.575, 0.610, 0.660);
pub const DANGER: Color = Color::from_rgb(0.831, 0.267, 0.220);

pub const RADIUS_XS: f32 = 3.0;
pub const RADIUS_SM: f32 = 6.0;
pub const RADIUS_MD: f32 = 10.0;

pub const DISPLAY_FONT: Font = Font {
    family: Family::Name("Cascadia Code"),
    weight: Weight::Semibold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub const BODY_FONT: Font = Font {
    family: Family::Name("Cascadia Code"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub fn mix(base: Color, accent: Color, amount: f32) -> Color {
    Color {
        r: base.r + (accent.r - base.r) * amount,
        g: base.g + (accent.g - base.g) * amount,
        b: base.b + (accent.b - base.b) * amount,
        a: base.a + (accent.a - base.a) * amount,
    }
}

pub fn soft_shadow(color: Color, y: f32, blur: f32) -> Shadow {
    Shadow {
        color,
        offset: Vector::new(0.0, y),
        blur_radius: blur,
    }
}

pub fn surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_1)),
        border: Border {
            radius: RADIUS_MD.into(),
            width: 1.0,
            color: DIVIDER,
        },
        ..Default::default()
    }
}

pub fn accent_strip(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(ACCENT)),
        border: Border {
            radius: 1.5.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(ACCENT)),
        text_color: SURFACE_0,
        border: Border {
            radius: RADIUS_SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
        snap: false,
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(mix(ACCENT, Color::WHITE, 0.12))),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(ACCENT_DIM)),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(ACCENT.scale_alpha(0.30))),
            text_color: TEXT.scale_alpha(0.50),
            ..base
        },
        button::Status::Active => base,
    }
}

pub fn ghost_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(SURFACE_2)),
        text_color: TEXT,
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: DIVIDER,
        },
        shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
        snap: false,
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(SURFACE_3)),
            border: Border {
                color: ACCENT.scale_alpha(0.60),
                ..base.border
            },
            text_color: ACCENT,
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(mix(SURFACE_2, Color::BLACK, 0.20))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(SURFACE_2.scale_alpha(0.45))),
            text_color: TEXT.scale_alpha(0.35),
            border: Border {
                color: DIVIDER.scale_alpha(0.40),
                ..base.border
            },
            ..base
        },
        button::Status::Active => base,
    }
}

pub fn danger_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(SURFACE_2)),
        text_color: DANGER,
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: DANGER.scale_alpha(0.55),
        },
        shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
        snap: false,
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(DANGER)),
            text_color: SURFACE_0,
            border: Border {
                color: DANGER,
                ..base.border
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(mix(DANGER, Color::BLACK, 0.20))),
            text_color: SURFACE_0,
            border: Border {
                color: DANGER,
                ..base.border
            },
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(SURFACE_2.scale_alpha(0.50))),
            text_color: DANGER.scale_alpha(0.40),
            border: Border {
                color: DANGER.scale_alpha(0.30),
                ..base.border
            },
            ..base
        },
        button::Status::Active => base,
    }
}

pub fn flat_input(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let active = text_input::Style {
        background: Background::Color(SURFACE_0),
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: DIVIDER,
        },
        icon: TEXT.scale_alpha(0.80),
        placeholder: TEXT_DIM,
        value: TEXT,
        selection: ACCENT.scale_alpha(0.30),
    };

    match status {
        text_input::Status::Hovered => text_input::Style {
            border: Border {
                color: mix(DIVIDER, ACCENT, 0.30),
                ..active.border
            },
            ..active
        },
        text_input::Status::Focused { .. } => text_input::Style {
            border: Border {
                width: 1.5,
                color: ACCENT,
                ..active.border
            },
            ..active
        },
        text_input::Status::Disabled => text_input::Style {
            value: TEXT.scale_alpha(0.40),
            placeholder: TEXT_DIM.scale_alpha(0.50),
            border: Border {
                color: DIVIDER.scale_alpha(0.40),
                ..active.border
            },
            ..active
        },
        text_input::Status::Active => active,
    }
}

pub fn flat_scrollable(_theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let rail = scrollable::Rail {
        background: Some(Background::Color(SURFACE_0.scale_alpha(0.55))),
        border: Border {
            radius: 4.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        scroller: scrollable::Scroller {
            background: Background::Color(DIVIDER),
            border: Border {
                radius: 4.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
    };

    let hot_rail = scrollable::Rail {
        scroller: scrollable::Scroller {
            background: Background::Color(ACCENT.scale_alpha(0.85)),
            ..rail.scroller
        },
        ..rail
    };

    match status {
        scrollable::Status::Active { .. } => scrollable::Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
            auto_scroll: scrollable::default(_theme, status).auto_scroll,
        },
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } => scrollable::Style {
            container: container::Style::default(),
            vertical_rail: if is_vertical_scrollbar_hovered {
                hot_rail
            } else {
                rail
            },
            horizontal_rail: if is_horizontal_scrollbar_hovered {
                hot_rail
            } else {
                rail
            },
            gap: None,
            auto_scroll: scrollable::default(_theme, status).auto_scroll,
        },
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            ..
        } => scrollable::Style {
            container: container::Style::default(),
            vertical_rail: if is_vertical_scrollbar_dragged {
                hot_rail
            } else {
                rail
            },
            horizontal_rail: if is_horizontal_scrollbar_dragged {
                hot_rail
            } else {
                rail
            },
            gap: None,
            auto_scroll: scrollable::default(_theme, status).auto_scroll,
        },
    }
}
