use iced::{Border, Color, widget::container};
use crate::games::wordly::mark::Mark;

pub fn marked_cell_style(mark: Mark) -> container::Style {
    match mark {
        Mark::Correct => container::Style {
            background: Some(Color::from_rgb(0.4, 0.0, 0.4).into()),
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.1, 0.8, 0.3),
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        Mark::Present => container::Style {
            background: Some(Color::from_rgb(0.0, 0.0, 0.2).into()),
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.0, 0.8, 0.0),
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        Mark::Absent => container::Style {
            background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.0, 0.0, 0.0),
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        Mark::Cursor => container::Style {
            background: Some(Color::from_rgb(0., 0., 0.15).into()),
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.2, 0.0, 0.2),
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        _ => container::Style {
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.12, 0.12, 0.1),
                radius: 6.0.into(),
            }
            ,
            ..Default::default()
        }
    }
}