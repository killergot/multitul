use crate::utils::style::{
    self, ACCENT, DIVIDER, RADIUS_SM, RADIUS_XS, SURFACE_0, SURFACE_1, SURFACE_2, TEAL, mix,
};
use iced::{
    Background, Border, Color, Theme,
    widget::{button, container, scrollable, text_input},
};

pub fn screen_shell(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_0)),
        ..Default::default()
    }
}

pub fn sidebar_panel(theme: &Theme) -> container::Style {
    style::surface(theme)
}

pub fn stage_panel(theme: &Theme) -> container::Style {
    style::surface(theme)
}

pub fn chat_panel(theme: &Theme) -> container::Style {
    style::surface(theme)
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_2)),
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: DIVIDER,
        },
        ..Default::default()
    }
}

pub fn accent_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_2)),
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: ACCENT.scale_alpha(0.55),
        },
        ..Default::default()
    }
}

pub fn word_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_1)),
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: mix(DIVIDER, TEAL, 0.30),
        },
        ..Default::default()
    }
}

pub fn system_bubble(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(mix(SURFACE_1, ACCENT, 0.06))),
        border: Border {
            radius: RADIUS_XS.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

pub fn player_bubble(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_2)),
        border: Border {
            radius: RADIUS_XS.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
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

pub fn primary_button(theme: &Theme, status: button::Status) -> button::Style {
    style::primary_button(theme, status)
}

pub fn secondary_button(theme: &Theme, status: button::Status) -> button::Style {
    style::ghost_button(theme, status)
}

pub fn danger_button(theme: &Theme, status: button::Status) -> button::Style {
    style::danger_button(theme, status)
}

pub fn game_input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let mut s = style::flat_input(theme, status);
    if let text_input::Status::Focused { .. } = status {
        s.border = Border {
            color: TEAL,
            ..s.border
        };
    }
    s
}

pub fn warm_input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    style::flat_input(theme, status)
}

pub fn panel_scrollable(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    style::flat_scrollable(theme, status)
}
