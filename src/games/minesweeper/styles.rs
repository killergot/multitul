use crate::games::minesweeper::model::{Cell, CellState, GameStatus};
use crate::utils::style::{
    self, ACCENT, DANGER, DIVIDER, RADIUS_SM, SURFACE_0, SURFACE_1, SURFACE_2, SURFACE_3, TEAL,
    TEXT, TEXT_DIM, mix,
};
use iced::{
    Background, Border, Color, Theme,
    widget::{button, container, scrollable as scroll_widget},
};

pub fn shell(theme: &Theme) -> container::Style {
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
            color: ACCENT.scale_alpha(0.7),
        },
        ..Default::default()
    }
}

pub fn board_frame(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE_0)),
        border: Border {
            radius: RADIUS_SM.into(),
            width: 1.0,
            color: DIVIDER.scale_alpha(0.8),
        },
        ..Default::default()
    }
}

pub fn cell(
    _theme: &Theme,
    cell: Cell,
    status: GameStatus,
    is_exploded: bool,
) -> container::Style {
    match cell.state {
        CellState::Hidden => container::Style {
            background: Some(Background::Color(SURFACE_2)),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: DIVIDER,
            },
            ..Default::default()
        },
        CellState::Flagged => container::Style {
            background: Some(Background::Color(mix(SURFACE_2, ACCENT, 0.15))),
            text_color: Some(ACCENT),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: ACCENT.scale_alpha(0.8),
            },
            ..Default::default()
        },
        CellState::Revealed => {
            let background = if cell.has_mine {
                if is_exploded {
                    DANGER
                } else {
                    mix(SURFACE_1, DANGER, 0.45)
                }
            } else {
                SURFACE_3
            };

            let border = if cell.has_mine {
                DANGER
            } else if matches!(status, GameStatus::Won) {
                TEAL
            } else {
                DIVIDER
            };

            container::Style {
                background: Some(Background::Color(background)),
                text_color: Some(TEXT),
                border: Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: border,
                },
                ..Default::default()
            }
        }
    }
}

pub fn number_color(cell: Cell) -> Color {
    if cell.state == CellState::Flagged {
        return ACCENT;
    }

    if cell.has_mine {
        return TEXT;
    }

    match cell.adjacent_mines {
        0 => TEXT_DIM,
        1 => Color::from_rgb(0.42, 0.72, 1.0),
        2 => Color::from_rgb(0.42, 0.82, 0.53),
        3 => Color::from_rgb(1.0, 0.52, 0.38),
        4 => Color::from_rgb(0.78, 0.58, 1.0),
        5 => Color::from_rgb(0.95, 0.42, 0.42),
        6 => Color::from_rgb(0.33, 0.86, 0.84),
        7 => Color::from_rgb(0.93, 0.93, 0.93),
        _ => Color::from_rgb(0.98, 0.75, 0.30),
    }
}

pub fn primary_button(theme: &Theme, status: button::Status) -> button::Style {
    style::primary_button(theme, status)
}

pub fn ghost_button(theme: &Theme, status: button::Status) -> button::Style {
    style::ghost_button(theme, status)
}

pub fn scrollable(theme: &Theme, status: scroll_widget::Status) -> scroll_widget::Style {
    style::flat_scrollable(theme, status)
}
