use unicode_segmentation::UnicodeSegmentation;

use iced::{
    Background, Border, Color, Element, Length, Shadow, Theme,
    widget::{button, column, container, row, text, text_input},
};

use super::attempt::Attempt;
use super::word_provider::WordProvider;

#[derive(Debug, Clone, Default)]
pub struct Wordly {
    state: WordlyState,
    proccess_game: WordlyGame,
    motion_phase: f32,
}

impl Wordly {
    pub fn view(&self) -> Element<'_, WordlyMessage> {
        match self.state {
            WordlyState::Menu => game_shell(
                "Wordly",
                "Угадай слово из пяти букв в шести попытках.",
                self.motion_phase,
                column![
                    info_panel(
                        "Как играть",
                        "Цвет клетки показывает попадание: точная буква, буква не на месте или промах.",
                        Color::from_rgb8(244, 114, 182),
                        self.motion_phase + 0.4,
                    ),
                    row![
                        button("Начать игру")
                            .style(primary_button_style())
                            .padding([14, 22])
                            .on_press(WordlyMessage::GoPlay),
                        button("На главную")
                            .style(ghost_button_style())
                            .padding([14, 22])
                            .on_press(WordlyMessage::GoHome),
                    ]
                    .spacing(16),
                ]
                .spacing(20),
            ),
            WordlyState::InGame => {
                let attempts = (0..6).map(|index| {
                    if let Some(attempt) = self.proccess_game.attempts.get(index) {
                        attempt_row(&attempt.word, Some(&attempt.marked))
                    } else {
                        attempt_row("", None)
                    }
                });

                let attempts_left = 6usize.saturating_sub(self.proccess_game.attempts.len());

                game_shell(
                    "Wordly",
                    "Сетка попыток и акцентные состояния без изменения игровой логики.",
                    self.motion_phase,
                    column![
                        row![
                            badge(
                                "Попыток осталось",
                                attempts_left.to_string(),
                                Color::from_rgb8(56, 189, 248),
                                self.motion_phase + 0.2,
                            ),
                            badge(
                                "Длина слова",
                                "5 букв",
                                Color::from_rgb8(244, 114, 182),
                                self.motion_phase + 1.1,
                            ),
                        ]
                        .spacing(12),
                        container(column(attempts).spacing(12))
                            .width(Length::Fill)
                            .padding([20, 20])
                            .style(panel_style(
                                Color::from_rgb8(8, 15, 30),
                                Color::from_rgb8(56, 189, 248),
                                self.motion_phase + 0.8,
                            )),
                        text_input("Введите слово", &self.proccess_game.current_input)
                            .style(input_style(self.motion_phase))
                            .on_input(WordlyMessage::InputChanged)
                            .on_submit(WordlyMessage::SubmitAttempt)
                            .padding([16, 18])
                            .size(24)
                            .width(320),
                        row![
                            button("Отправить")
                                .style(primary_button_style())
                                .padding([14, 22])
                                .on_press(WordlyMessage::SubmitAttempt),
                            button("На главную")
                                .style(ghost_button_style())
                                .padding([14, 22])
                                .on_press(WordlyMessage::GoHome),
                        ]
                        .spacing(16),
                    ]
                    .spacing(20),
                )
            }
            WordlyState::FinishedWin => game_shell(
                "Финиш",
                "Победа вынесена в отдельную акцентную карточку.",
                self.motion_phase,
                column![
                    result_panel(
                        "Победа",
                        format!(
                            "Угадано за {} попыток. Слово: {}",
                            self.proccess_game.attempts.len(),
                            self.proccess_game.word
                        ),
                        Color::from_rgb8(34, 197, 94),
                        self.motion_phase,
                    ),
                    button("В меню")
                        .style(primary_button_style())
                        .padding([14, 22])
                        .on_press(WordlyMessage::GoHome),
                ]
                .spacing(20),
            ),
            WordlyState::FinishedLose => game_shell(
                "Финиш",
                "Поражение тоже оформлено чисто и контрастно.",
                self.motion_phase,
                column![
                    result_panel(
                        "Не угадано",
                        format!(
                            "6 попыток использованы. Загаданное слово: {}",
                            self.proccess_game.word
                        ),
                        Color::from_rgb8(248, 113, 113),
                        self.motion_phase,
                    ),
                    row![
                        button("На главную")
                            .style(ghost_button_style())
                            .padding([14, 22])
                            .on_press(WordlyMessage::GoHome),
                        button("В меню")
                            .style(primary_button_style())
                            .padding([14, 22])
                            .on_press(WordlyMessage::GoHome),
                    ]
                    .spacing(16),
                ]
                .spacing(20),
            ),
        }
    }

    pub fn update(&mut self, message: WordlyMessage) {
        match message {
            WordlyMessage::GoPlay => {
                self.state = WordlyState::InGame;
                self.proccess_game = WordlyGame::new();
            }
            WordlyMessage::SubmitAttempt => {
                if self.proccess_game.current_input == self.proccess_game.word {
                    self.state = WordlyState::FinishedWin;
                    self.proccess_game.update(message);
                } else if self.proccess_game.attempts.len() == 6 {
                    self.state = WordlyState::FinishedLose;
                } else {
                    self.proccess_game.update(message);
                }
            }
            _ => {
                self.proccess_game.update(message);
            }
        }
    }

    pub fn set_motion(&mut self, phase: f32) {
        self.motion_phase = phase;
    }
}

#[derive(Debug, Clone, Default)]
struct WordlyGame {
    word: String,
    attempts: Vec<Attempt>,
    current_input: String,
}

impl WordlyGame {
    pub fn new() -> WordlyGame {
        WordlyGame {
            word: WordProvider::get_one_word_5_ru(),
            attempts: vec![],
            current_input: String::new(),
        }
    }

    pub fn update(&mut self, message: WordlyMessage) {
        match message {
            WordlyMessage::InputChanged(txt) => {
                if txt.graphemes(true).count() <= 5 {
                    self.current_input = txt;
                }
            }
            WordlyMessage::SubmitAttempt => {
                if self.current_input.graphemes(true).count() == 5 {
                    self.attempts
                        .push(Attempt::new(self.word.clone(), self.current_input.clone()));
                    self.current_input.clear();
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum WordlyMessage {
    GoHome,
    GoPlay,
    InputChanged(String),
    SubmitAttempt,
}

#[derive(Debug, Clone, Default)]
pub enum WordlyState {
    #[default]
    Menu,
    InGame,
    FinishedLose,
    FinishedWin,
}

fn game_shell<'a>(
    title: &'a str,
    subtitle: &'a str,
    phase: f32,
    body: impl Into<Element<'a, WordlyMessage>>,
) -> Element<'a, WordlyMessage> {
    container(
        column![
            animated_meter(
                phase,
                [
                    Color::from_rgb8(244, 114, 182),
                    Color::from_rgb8(56, 189, 248),
                    Color::from_rgb8(34, 197, 94),
                ],
            ),
            text(title).size(42).color(Color::WHITE),
            text(subtitle)
                .size(18)
                .color(Color::from_rgb8(191, 219, 254)),
            container(body)
                .width(Length::Fill)
                .padding([24, 24])
                .style(panel_style(
                    Color::from_rgb8(10, 14, 28),
                    Color::from_rgb8(244, 114, 182),
                    phase,
                )),
        ]
        .spacing(22)
        .max_width(720),
    )
    .width(Length::Fill)
    .into()
}

fn info_panel<'a>(
    title: &'a str,
    body: &'a str,
    accent: Color,
    phase: f32,
) -> Element<'a, WordlyMessage> {
    container(
        column![
            text(title).size(20).color(Color::WHITE),
            text(body)
                .size(16)
                .color(Color::from_rgb8(148, 163, 184)),
        ]
        .spacing(8),
    )
    .padding([18, 18])
    .width(Length::Fill)
    .style(panel_style(Color::from_rgb8(8, 15, 30), accent, phase))
    .into()
}

fn badge<'a>(
    label: &'a str,
    value: impl ToString,
    accent: Color,
    phase: f32,
) -> Element<'a, WordlyMessage> {
    container(
        column![
            text(label).size(14).color(Color::from_rgb8(148, 163, 184)),
            text(value.to_string()).size(22).color(Color::WHITE),
        ]
        .spacing(6),
    )
    .padding([14, 16])
    .style(panel_style(Color::from_rgb8(8, 15, 30), accent, phase))
    .into()
}

fn result_panel<'a>(
    title: &'a str,
    body: String,
    accent: Color,
    phase: f32,
) -> Element<'a, WordlyMessage> {
    container(
        column![
            text(title).size(28).color(Color::WHITE),
            text(body).size(18).color(Color::from_rgb8(226, 232, 240)),
        ]
        .spacing(10),
    )
    .padding([22, 22])
    .width(Length::Fill)
    .style(panel_style(Color::from_rgb8(8, 15, 30), accent, phase))
    .into()
}

fn attempt_row<'a>(word: &'a str, marks: Option<&'a [u8; 5]>) -> Element<'a, WordlyMessage> {
    let letters: Vec<String> = word
        .graphemes(true)
        .map(|grapheme| grapheme.to_uppercase())
        .collect();

    let cells = (0..5).map(|index| {
        let grapheme = letters.get(index).cloned().unwrap_or_default();
        let mark = marks.map(|values| values[index]);
        letter_cell(grapheme, mark)
    });

    row(cells).spacing(12).into()
}

fn letter_cell(value: String, mark: Option<u8>) -> Element<'static, WordlyMessage> {
    let (background, border_color, text_color) = match mark {
        Some(2) => (
            Color::from_rgb8(74, 222, 128),
            Color::from_rgb8(187, 247, 208),
            Color::from_rgb8(5, 46, 22),
        ),
        Some(1) => (
            Color::from_rgb8(244, 114, 182),
            Color::from_rgb8(251, 207, 232),
            Color::WHITE,
        ),
        Some(_) => (
            Color::from_rgb8(30, 41, 59),
            Color::from_rgb8(71, 85, 105),
            Color::from_rgb8(148, 163, 184),
        ),
        None => (
            Color::from_rgb8(15, 23, 42),
            Color::from_rgba8(148, 163, 184, 0.22),
            Color::from_rgb8(71, 85, 105),
        ),
    };

    let label = if value.is_empty() { " ".to_string() } else { value };

    container(text(label).size(28).color(text_color))
        .width(62)
        .height(62)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(background)),
            text_color: Some(text_color),
            border: Border {
                radius: 18.0.into(),
                width: 1.0,
                color: border_color,
            },
            shadow: Shadow {
                color: border_color.scale_alpha(0.18 + 0.08 * border_color.a),
                offset: [0.0, 8.0].into(),
                blur_radius: 18.0,
            },
            snap: false,
        })
        .into()
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
            radius: 24.0.into(),
            width: 1.0,
            color: border_color.scale_alpha(0.5),
        },
        shadow: Shadow {
            color: border_color.scale_alpha(0.14 + 0.14 * phase.sin().abs()),
            offset: [0.0, 16.0 + phase.cos().abs() * 10.0].into(),
            blur_radius: 24.0 + phase.sin().abs() * 16.0,
        },
        snap: false,
    }
}

fn primary_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(
                (if hovered {
                    Color::from_rgb8(236, 72, 153)
                } else {
                    Color::from_rgb8(219, 39, 119)
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
                color: Color::from_rgba8(219, 39, 119, 0.35),
                offset: [0.0, if hovered { 10.0 } else { 6.0 }].into(),
                blur_radius: if hovered { 20.0 } else { 12.0 },
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
                    Color::from_rgba8(244, 114, 182, 0.14)
                } else {
                    Color::from_rgba8(15, 23, 42, 0.0)
                })
                .into(),
            ),
            text_color: Color::from_rgb8(251, 207, 232),
            border: Border {
                radius: 16.0.into(),
                width: 1.0,
                color: Color::from_rgba8(244, 114, 182, if hovered { 0.52 } else { 0.28 }),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

fn input_style(phase: f32) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_theme, status| {
        let is_focused = matches!(status, text_input::Status::Focused { .. });

        text_input::Style {
            background: Background::Color(Color::from_rgb8(8, 15, 30)),
            border: Border {
                radius: 18.0.into(),
                width: 1.0,
                color: if is_focused {
                    Color::from_rgba(
                        244.0 / 255.0,
                        114.0 / 255.0,
                        182.0 / 255.0,
                        0.72 + 0.18 * phase.sin().abs(),
                    )
                } else {
                    Color::from_rgba8(148, 163, 184, 0.28)
                },
            },
            icon: Color::from_rgb8(148, 163, 184),
            placeholder: Color::from_rgb8(100, 116, 139),
            value: Color::WHITE,
            selection: Color::from_rgba8(244, 114, 182, 0.28),
        }
    }
}

fn animated_meter<'a>(phase: f32, accents: [Color; 3]) -> Element<'a, WordlyMessage> {
    let bars = accents.into_iter().enumerate().map(|(index, color)| {
        let local = phase + index as f32 * 0.85;
        let width = 72.0 + local.sin().abs() * 132.0;

        container("")
            .width(width)
            .height(6)
            .style(move |_theme| container::Style {
                background: Some(color.scale_alpha(0.56 + local.cos().abs() * 0.3).into()),
                text_color: None,
                border: Border {
                    radius: 999.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: color.scale_alpha(0.16 + local.sin().abs() * 0.2),
                    offset: [0.0, 0.0].into(),
                    blur_radius: 10.0 + local.cos().abs() * 10.0,
                },
                snap: false,
            })
            .into()
    });

    row(bars).spacing(10).into()
}
