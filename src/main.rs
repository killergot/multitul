mod games;

use crate::games::wordly::{Wordly, WordlyMessage};
use iced::{
    Alignment, Border, Color, Element, Length, Shadow, Subscription, Theme, window,
    widget::{button, column, container, row, text},
};
use iced::time::Instant;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .subscription(subscription)
        .theme(theme)
        .run()
}

struct App {
    screen: Screen,
    motion_phase: f32,
    last_tick: Option<Instant>,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Main,
            motion_phase: 0.0,
            last_tick: None,
        }
    }

    fn update(app: &mut Self, message: Message) {
        match message {
            Message::Tick(now) => {
                let delta = app
                    .last_tick
                    .map(|last| now.saturating_duration_since(last).as_secs_f32())
                    .unwrap_or(0.016)
                    .min(0.05);

                app.last_tick = Some(now);
                app.motion_phase = (app.motion_phase + delta * 1.15) % (std::f32::consts::TAU);

                if let Screen::Wordly(wordly) = &mut app.screen {
                    wordly.set_motion(app.motion_phase);
                }
            }
            Message::Counter(msg) => match msg {
                CounterMessage::Increment => {
                    if let Screen::Counter(counter) = &mut app.screen {
                        counter.value += 1;
                    }
                }
                CounterMessage::Decrement => {
                    if let Screen::Counter(counter) = &mut app.screen {
                        counter.value -= 1;
                    }
                }
            },
            Message::Wordly(msg) => match msg {
                WordlyMessage::GoHome => {
                    app.screen = Screen::Main;
                }
                msg => {
                    if let Screen::Wordly(wordly) = &mut app.screen {
                        wordly.update(msg);
                    }
                }
            },
            Message::SwitchTo(mut msg) => {
                if let Screen::Wordly(wordly) = &mut msg {
                    wordly.set_motion(app.motion_phase);
                }
                app.screen = msg;
            }
        }
    }

    fn view(app: &Self) -> Element<'_, Message> {
        let content = match &app.screen {
            Screen::Counter(counter) => {
                let value_card = container(
                    column![
                        text("Counter")
                            .size(18)
                            .color(Color::from_rgb8(148, 163, 184)),
                        text(counter.value).size(76).color(Color::WHITE),
                    ]
                    .spacing(10)
                    .align_x(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([28, 32])
                .style(panel_style(
                    Color::from_rgb8(15, 23, 42),
                    Color::from_rgb8(56, 189, 248),
                    app.motion_phase + 0.6,
                ));

                hero_shell(
                    "Rust Multitul",
                    "Контрастный счётчик с крупной типографикой и насыщенными акцентами.",
                    column![
                        value_card,
                        row![
                            button("Увеличить")
                                .style(primary_button_style())
                                .padding([14, 20])
                                .on_press(Message::Counter(CounterMessage::Increment)),
                            button("Уменьшить")
                                .style(secondary_button_style())
                                .padding([14, 20])
                                .on_press(Message::Counter(CounterMessage::Decrement)),
                        ]
                        .spacing(16),
                        button("На главную")
                            .style(ghost_button_style())
                            .padding([14, 20])
                            .on_press(Message::SwitchTo(Screen::Main)),
                    ]
                    .spacing(20)
                    .max_width(560),
                    app.motion_phase,
                )
            }
            Screen::Wordly(wordly_game) => wordly_game.view().map(Message::Wordly),
            Screen::Main => hero_shell(
                "Rust Multitul",
                "Набор мини-приложений в более кинематографичной и собранной оболочке.",
                column![
                    feature_button(
                        "Counter",
                        "Простой счётчик, оформленный как акцентная панель.",
                        Message::SwitchTo(Screen::Counter(Counter::default())),
                        Color::from_rgb8(56, 189, 248),
                        app.motion_phase + 0.2,
                    ),
                    feature_button(
                        "Wordly",
                        "Словесная игра с более живой сеткой и сильнее выраженными состояниями.",
                        Message::SwitchTo(Screen::Wordly(Wordly::default())),
                        Color::from_rgb8(244, 114, 182),
                        app.motion_phase + 1.4,
                    ),
                ]
                .spacing(16)
                .max_width(620),
                app.motion_phase,
            ),
        };

        container(
            container(content)
                .width(Length::Fill)
                .max_width(960)
                .padding([32, 20]),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |theme| root_style(theme, app.motion_phase))
        .into()
    }
}

#[derive(Debug, Clone)]
enum Screen {
    Counter(Counter),
    Wordly(Wordly),
    Main,
}

#[derive(Debug, Clone, Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone)]
enum Message {
    SwitchTo(Screen),
    Counter(CounterMessage),
    Wordly(WordlyMessage),
    Tick(Instant),
}

#[derive(Debug, Clone)]
enum CounterMessage {
    Increment,
    Decrement,
}

fn theme(_app: &App) -> Theme {
    Theme::Dark
}

fn subscription(_app: &App) -> Subscription<Message> {
    window::frames().map(Message::Tick)
}

fn hero_shell<'a>(
    title: &'a str,
    subtitle: &'a str,
    body: impl Into<Element<'a, Message>>,
    phase: f32,
) -> Element<'a, Message> {
    let halo = 0.5 + 0.5 * phase.sin();

    container(
        column![
            animated_meter(
                phase,
                [
                    Color::from_rgb8(56, 189, 248),
                    Color::from_rgb8(96, 165, 250),
                    Color::from_rgb8(244, 114, 182),
                ],
            ),
            text(title).size(46).color(Color::WHITE),
            text(subtitle)
                .size(18)
                .color(Color::from_rgb8(191, 219, 254)),
            container(body)
                .width(Length::Fill)
                .padding([24, 24])
                .style(panel_style(
                    Color::from_rgb8(10, 14, 28),
                    Color::from_rgba(
                        96.0 / 255.0,
                        165.0 / 255.0,
                        250.0 / 255.0,
                        0.45 + halo * 0.35,
                    ),
                    phase,
                )),
        ]
        .spacing(24),
    )
    .width(Length::Fill)
    .into()
}

fn feature_button<'a>(
    title: &'a str,
    description: &'a str,
    message: Message,
    accent: Color,
    phase: f32,
) -> Element<'a, Message> {
    let shimmer = 0.45 + 0.55 * phase.sin().abs();

    button(
        row![
            container(text("●").size(20).color(accent.scale_alpha(0.75 + shimmer * 0.25)))
                .width(36)
                .center_x(Length::Shrink)
                .center_y(Length::Shrink),
            column![
                text(title).size(28).color(Color::WHITE),
                text(description)
                    .size(16)
                    .color(Color::from_rgb8(148, 163, 184)),
            ]
            .spacing(8)
            .width(Length::Fill),
            text("Открыть").size(16).color(accent),
        ]
        .align_y(Alignment::Center)
        .spacing(14),
    )
    .width(Length::Fill)
    .padding([18, 20])
    .style(move |_theme, status| {
        let active = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(
                (if active {
                    accent.scale_alpha(0.20 + shimmer * 0.12)
                } else {
                    Color::from_rgba8(15, 23, 42, 0.82 + shimmer * 0.08)
                })
                .into(),
            ),
            text_color: Color::WHITE,
            border: Border {
                radius: 22.0.into(),
                width: if active { 2.0 } else { 1.0 },
                color: if active {
                    accent
                } else {
                    Color::from_rgba8(148, 163, 184, 0.22)
                },
            },
            shadow: Shadow {
                color: accent.scale_alpha(0.12 + shimmer * 0.18),
                offset: if active {
                    [0.0, 10.0].into()
                } else {
                    [0.0, 4.0].into()
                },
                blur_radius: if active { 28.0 } else { 16.0 + shimmer * 8.0 },
            },
            snap: false,
        }
    })
    .on_press(message)
    .into()
}

fn root_style(_theme: &Theme, phase: f32) -> container::Style {
    let glow = 0.5 + 0.5 * phase.sin();

    container::Style {
        background: Some(
            Color::from_rgba(
                (2.0 + glow * 5.0) / 255.0,
                (6.0 + glow * 8.0) / 255.0,
                (23.0 + glow * 22.0) / 255.0,
                1.0,
            )
            .into(),
        ),
        text_color: Some(Color::WHITE),
        snap: false,
        ..Default::default()
    }
}

fn panel_style(
    bg: Color,
    border_color: Color,
    phase: f32,
) -> impl Fn(&Theme) -> container::Style {
    move |_theme: &Theme| container::Style {
        background: Some(bg.into()),
        text_color: Some(Color::WHITE),
        border: Border {
            radius: 28.0.into(),
            width: 1.0,
            color: border_color.scale_alpha(0.55),
        },
        shadow: Shadow {
            color: border_color.scale_alpha(0.14 + 0.10 * phase.sin().abs()),
            offset: [0.0, 18.0 + phase.cos().abs() * 8.0].into(),
            blur_radius: 30.0 + phase.sin().abs() * 18.0,
        },
        snap: false,
    }
}

fn animated_meter<'a>(phase: f32, accents: [Color; 3]) -> Element<'a, Message> {
    let bars = accents.into_iter().enumerate().map(|(index, color)| {
        let local = phase + index as f32 * 0.9;
        let width = 80.0 + local.sin().abs() * 140.0;

        container("")
            .width(width)
            .height(6)
            .style(move |_theme| container::Style {
                background: Some(color.scale_alpha(0.55 + local.cos().abs() * 0.35).into()),
                text_color: None,
                border: Border {
                    radius: 999.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: color.scale_alpha(0.18 + local.sin().abs() * 0.22),
                    offset: [0.0, 0.0].into(),
                    blur_radius: 12.0 + local.cos().abs() * 10.0,
                },
                snap: false,
            })
            .into()
    });

    row(bars).spacing(10).into()
}

fn primary_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(
                (if hovered {
                    Color::from_rgb8(14, 165, 233)
                } else {
                    Color::from_rgb8(2, 132, 199)
                })
                .into(),
            ),
            text_color: Color::WHITE,
            border: Border {
                radius: 16.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: Shadow {
                color: Color::from_rgba8(14, 165, 233, 0.35),
                offset: [0.0, if hovered { 10.0 } else { 6.0 }].into(),
                blur_radius: if hovered { 20.0 } else { 12.0 },
            },
            snap: false,
        }
    }
}

fn secondary_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(
                (if hovered {
                    Color::from_rgb8(30, 41, 59)
                } else {
                    Color::from_rgb8(15, 23, 42)
                })
                .into(),
            ),
            text_color: Color::WHITE,
            border: Border {
                radius: 16.0.into(),
                width: 1.0,
                color: Color::from_rgba8(148, 163, 184, if hovered { 0.45 } else { 0.28 }),
            },
            shadow: Shadow {
                color: Color::from_rgba8(15, 23, 42, 0.35),
                offset: [0.0, 6.0].into(),
                blur_radius: 14.0,
            },
            snap: false,
        }
    }
}

fn ghost_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(
                (if hovered {
                    Color::from_rgba8(59, 130, 246, 0.16)
                } else {
                    Color::from_rgba8(15, 23, 42, 0.0)
                })
                .into(),
            ),
            text_color: Color::from_rgb8(191, 219, 254),
            border: Border {
                radius: 16.0.into(),
                width: 1.0,
                color: Color::from_rgba8(96, 165, 250, if hovered { 0.55 } else { 0.28 }),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }
}
