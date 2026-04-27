use crate::utils::style::{mix, soft_shadow};
use iced::{
    Background, Border, Color, Theme, Vector,
    widget::{button, container, scrollable, text_input},
};

const INK: Color = Color::from_rgb(0.07, 0.09, 0.13);
const INK_LIGHT: Color = Color::from_rgb(0.11, 0.13, 0.18);
const PANEL: Color = Color::from_rgb(0.14, 0.16, 0.21);
const PANEL_RAISED: Color = Color::from_rgb(0.17, 0.20, 0.26);
const COPPER: Color = Color::from_rgb(0.86, 0.50, 0.27);
const COPPER_LIGHT: Color = Color::from_rgb(0.96, 0.72, 0.45);
const TEAL: Color = Color::from_rgb(0.24, 0.70, 0.66);
const TEAL_LIGHT: Color = Color::from_rgb(0.62, 0.89, 0.84);
const CREAM: Color = Color::from_rgb(0.96, 0.93, 0.87);
const DANGER: Color = Color::from_rgb(0.80, 0.32, 0.28);
const LINE: Color = Color::from_rgba(0.95, 0.90, 0.82, 0.10);

const RADIUS_LG: f32 = 20.0;
const RADIUS_MD: f32 = 16.0;
const RADIUS_SM: f32 = 12.0;

pub fn screen_shell(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(INK)),
        border: Border {
            radius: 26.0.into(),
            width: 1.0,
            color: LINE,
        },
        shadow: soft_shadow(Color::BLACK.scale_alpha(0.30), 6.0, 24.0),
        ..Default::default()
    }
}

pub fn sidebar_panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(INK, COPPER, 0.07))),
        border: Border {
            radius: RADIUS_LG.into(),
            width: 1.0,
            color: mix(COPPER, CREAM, 0.18).scale_alpha(0.55),
        },
        shadow: soft_shadow(COPPER.scale_alpha(0.10), 4.0, 18.0),
        ..Default::default()
    }
}

pub fn stage_panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(PANEL)),
        border: Border {
            radius: RADIUS_LG.into(),
            width: 1.0,
            color: mix(TEAL, CREAM, 0.10).scale_alpha(0.55),
        },
        shadow: soft_shadow(Color::BLACK.scale_alpha(0.18), 4.0, 18.0),
        ..Default::default()
    }
}

pub fn chat_panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(PANEL, TEAL, 0.06))),
        border: Border {
            radius: RADIUS_LG.into(),
            width: 1.0,
            color: mix(TEAL, CREAM, 0.16).scale_alpha(0.55),
        },
        shadow: soft_shadow(TEAL.scale_alpha(0.10), 4.0, 18.0),
        ..Default::default()
    }
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(PANEL_RAISED)),
        border: Border {
            radius: RADIUS_MD.into(),
            width: 1.0,
            color: LINE,
        },
        shadow: soft_shadow(Color::BLACK.scale_alpha(0.10), 2.0, 8.0),
        ..Default::default()
    }
}

pub fn accent_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(PANEL_RAISED, COPPER, 0.10))),
        border: Border {
            radius: RADIUS_MD.into(),
            width: 1.0,
            color: mix(COPPER, CREAM, 0.20).scale_alpha(0.65),
        },
        shadow: soft_shadow(COPPER.scale_alpha(0.10), 3.0, 12.0),
        ..Default::default()
    }
}

pub fn word_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(PANEL_RAISED, TEAL, 0.10))),
        border: Border {
            radius: RADIUS_MD.into(),
            width: 1.0,
            color: mix(TEAL_LIGHT, CREAM, 0.10).scale_alpha(0.55),
        },
        shadow: soft_shadow(TEAL.scale_alpha(0.10), 3.0, 12.0),
        ..Default::default()
    }
}

pub fn system_bubble(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(INK_LIGHT, COPPER, 0.12))),
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: mix(COPPER, CREAM, 0.16).scale_alpha(0.45),
        },
        ..Default::default()
    }
}

pub fn player_bubble(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(INK_LIGHT, TEAL, 0.14))),
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: mix(TEAL_LIGHT, CREAM, 0.10).scale_alpha(0.45),
        },
        ..Default::default()
    }
}

pub fn round_card(theme: &Theme, is_match: bool) -> container::Style {
    if is_match {
        accent_card(theme)
    } else {
        card(theme)
    }
}

pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(COPPER)),
        text_color: INK,
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: mix(COPPER_LIGHT, CREAM, 0.30).scale_alpha(0.75),
        },
        shadow: soft_shadow(COPPER.scale_alpha(0.22), 3.0, 10.0),
        snap: false,
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(COPPER_LIGHT)),
            border: Border {
                color: CREAM.scale_alpha(0.85),
                ..base.border
            },
            shadow: soft_shadow(COPPER.scale_alpha(0.30), 5.0, 14.0),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(mix(COPPER, INK, 0.20))),
            shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(COPPER.scale_alpha(0.32))),
            text_color: CREAM.scale_alpha(0.50),
            border: Border {
                color: base.border.color.scale_alpha(0.30),
                ..base.border
            },
            shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
            ..base
        },
        button::Status::Active => base,
    }
}

pub fn secondary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(mix(PANEL_RAISED, TEAL, 0.12))),
        text_color: CREAM,
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: mix(TEAL_LIGHT, CREAM, 0.10).scale_alpha(0.55),
        },
        shadow: soft_shadow(TEAL.scale_alpha(0.12), 3.0, 10.0),
        snap: false,
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(mix(PANEL_RAISED, TEAL_LIGHT, 0.20))),
            border: Border {
                color: mix(TEAL_LIGHT, CREAM, 0.30).scale_alpha(0.75),
                ..base.border
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(mix(PANEL_RAISED, INK, 0.20))),
            shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(PANEL_RAISED.scale_alpha(0.40))),
            text_color: CREAM.scale_alpha(0.40),
            border: Border {
                color: base.border.color.scale_alpha(0.25),
                ..base.border
            },
            shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
            ..base
        },
        button::Status::Active => base,
    }
}

pub fn danger_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(DANGER)),
        text_color: CREAM,
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: mix(DANGER, CREAM, 0.20).scale_alpha(0.65),
        },
        shadow: soft_shadow(DANGER.scale_alpha(0.20), 3.0, 10.0),
        snap: false,
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(mix(DANGER, COPPER_LIGHT, 0.18))),
            border: Border {
                color: CREAM.scale_alpha(0.85),
                ..base.border
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(mix(DANGER, INK, 0.22))),
            shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(DANGER.scale_alpha(0.36))),
            text_color: CREAM.scale_alpha(0.40),
            border: Border {
                color: base.border.color.scale_alpha(0.25),
                ..base.border
            },
            shadow: soft_shadow(Color::TRANSPARENT, 0.0, 0.0),
            ..base
        },
        button::Status::Active => base,
    }
}

pub fn game_input(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let active = text_input::Style {
        background: Background::Color(mix(INK_LIGHT, TEAL, 0.08)),
        border: Border {
            radius: RADIUS_MD.into(),
            width: 1.0,
            color: mix(TEAL, CREAM, 0.10).scale_alpha(0.55),
        },
        icon: CREAM.scale_alpha(0.80),
        placeholder: CREAM.scale_alpha(0.42),
        value: CREAM,
        selection: TEAL.scale_alpha(0.32),
    };

    match status {
        text_input::Status::Hovered => text_input::Style {
            border: Border {
                color: mix(TEAL_LIGHT, CREAM, 0.24).scale_alpha(0.70),
                ..active.border
            },
            ..active
        },
        text_input::Status::Focused { .. } => text_input::Style {
            background: Background::Color(mix(INK_LIGHT, TEAL, 0.14)),
            border: Border {
                width: 1.5,
                color: mix(TEAL_LIGHT, CREAM, 0.38),
                ..active.border
            },
            ..active
        },
        text_input::Status::Disabled => text_input::Style {
            background: Background::Color(INK_LIGHT.scale_alpha(0.40)),
            value: CREAM.scale_alpha(0.40),
            placeholder: CREAM.scale_alpha(0.30),
            border: Border {
                color: active.border.color.scale_alpha(0.25),
                ..active.border
            },
            ..active
        },
        text_input::Status::Active => active,
    }
}

pub fn warm_input(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let active = text_input::Style {
        background: Background::Color(mix(INK_LIGHT, COPPER, 0.06)),
        border: Border {
            radius: RADIUS_MD.into(),
            width: 1.0,
            color: mix(COPPER, CREAM, 0.14).scale_alpha(0.55),
        },
        icon: CREAM.scale_alpha(0.80),
        placeholder: CREAM.scale_alpha(0.42),
        value: CREAM,
        selection: COPPER.scale_alpha(0.32),
    };

    match status {
        text_input::Status::Hovered => text_input::Style {
            border: Border {
                color: mix(COPPER_LIGHT, CREAM, 0.26).scale_alpha(0.70),
                ..active.border
            },
            ..active
        },
        text_input::Status::Focused { .. } => text_input::Style {
            background: Background::Color(mix(INK_LIGHT, COPPER, 0.12)),
            border: Border {
                width: 1.5,
                color: mix(COPPER_LIGHT, CREAM, 0.38),
                ..active.border
            },
            ..active
        },
        text_input::Status::Disabled => text_input::Style {
            background: Background::Color(INK_LIGHT.scale_alpha(0.40)),
            value: CREAM.scale_alpha(0.40),
            placeholder: CREAM.scale_alpha(0.30),
            border: Border {
                color: active.border.color.scale_alpha(0.25),
                ..active.border
            },
            ..active
        },
        text_input::Status::Active => active,
    }
}

pub fn panel_scrollable(_theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let rail = scrollable::Rail {
        background: Some(Background::Color(INK_LIGHT.scale_alpha(0.55))),
        border: Border {
            radius: 8.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        scroller: scrollable::Scroller {
            background: Background::Color(TEAL.scale_alpha(0.75)),
            border: Border {
                radius: 8.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
    };

    let hot_rail = scrollable::Rail {
        scroller: scrollable::Scroller {
            background: Background::Color(COPPER.scale_alpha(0.85)),
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
            auto_scroll: scrollable::default(&_theme.clone(), status).auto_scroll,
        },
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } => scrollable::Style {
            container: container::Style::default(),
            vertical_rail: if is_vertical_scrollbar_hovered { hot_rail } else { rail },
            horizontal_rail: if is_horizontal_scrollbar_hovered { hot_rail } else { rail },
            gap: None,
            auto_scroll: scrollable::default(&_theme.clone(), status).auto_scroll,
        },
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            ..
        } => scrollable::Style {
            container: container::Style::default(),
            vertical_rail: if is_vertical_scrollbar_dragged { hot_rail } else { rail },
            horizontal_rail: if is_horizontal_scrollbar_dragged { hot_rail } else { rail },
            gap: None,
            auto_scroll: scrollable::default(&_theme.clone(), status).auto_scroll,
        },
    }
}

pub fn split_handle(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(INK, COPPER, 0.18))),
        border: Border {
            radius: 4.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            color: COPPER.scale_alpha(0.25),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 6.0,
        },
        ..Default::default()
    }
}

pub fn bottom_dock(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(INK, PANEL, 0.55))),
        border: Border {
            radius: 18.0.into(),
            width: 1.0,
            color: LINE,
        },
        shadow: soft_shadow(Color::BLACK.scale_alpha(0.25), -2.0, 16.0),
        ..Default::default()
    }
}
