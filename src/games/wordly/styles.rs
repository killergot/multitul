use iced::{Border, Color, widget::container};

pub fn key_style(mark: u8) -> container::Style {
    match mark {
        2 => container::Style {
            background: Some(Color::from_rgb(0.4, 0.0, 0.4).into()),
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.1, 0.8, 0.3),
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        1 => container::Style {
            background: Some(Color::from_rgb(0.0, 0.0, 0.2).into()),
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.0, 0.8, 0.0),
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        0 => container::Style {
            background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.0, 0.0, 0.0),
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        _ => Default::default(),
    }
}