use unicode_segmentation::UnicodeSegmentation;

use iced::{Alignment, Element, Length};
use iced::widget::{Space, button, column, container, row, text};
use crate::games::wordly::mark::Mark;
use crate::KeyMessage;
use super::attempt::Attempt;
use super::consts::*;
use super::styles;
use super::word_provider::WordProvider;

#[derive(Debug, Clone, Default)]
pub struct Wordly {
    state: WordlyState,
    proccess_game: WordlyGame,
    all_worlds: Vec<String>,
}

fn key_widget<'a>(symbol: &'a str, mark: Mark) -> Element<'a, WordlyMessage> {
    button(text(symbol).size(20).style(styles::title_style))
        .height(KEY_WIDGET_SIZE)
        .width(KEY_WIDGET_SIZE)
        .style(move |theme, status| styles::keyboard_button_style(theme, status, mark))
        .on_press(WordlyMessage::KeyboardClicked(symbol.to_string()))
        .into()
}

fn replace_by_index(current_input: &mut String, cursor: usize, new_sym: &str) -> String {
    current_input
        .graphemes(true)
        .enumerate()
        .map(|(i, sym)| {
            if i == cursor {
                new_sym.to_string()
            } else {
                sym.to_string()
            }
        })
        .collect()
}

fn attempt_widget<'a>(attempts: &Vec<Attempt>) -> Element<'a, WordlyMessage> {
    column(attempts.iter().map(|attempt| {
        row(attempt
            .word
            .chars()
            .zip(attempt.marked)
            .map(|(character, mark)| {
                container(text(character.to_string()).size(32).style(styles::title_style))
                    .height(CHAR_WIDGET_SIZE)
                    .width(CHAR_WIDGET_SIZE)
                    .center(CHAR_WIDGET_SIZE)
                    .style(move |_| styles::marked_cell_style(mark))
                    .into()
            }))
        .spacing(BASE_SPACE)
        .into()
    }))
    .spacing(BASE_SPACE + 3)
    .into()
}

fn input_attempt_widget<'a>(input_text: &'a str, cursor: usize) -> Element<'a, WordlyMessage> {
    row(input_text.graphemes(true).enumerate().map(|(i, sym)| {
        let mark = if i == cursor {
            Mark::Cursor
        } else {
            Mark::Unknown
        };

        container(text(sym).size(32).style(styles::title_style))
            .height(CHAR_WIDGET_SIZE)
            .width(CHAR_WIDGET_SIZE)
            .center(CHAR_WIDGET_SIZE)
            .style(move |_| styles::marked_cell_style(mark))
            .into()
    }))
    .spacing(BASE_SPACE)
    .into()
}

fn keyboard_widget<'a>(keyboard: &'a Vec<(String, Mark)>) -> Element<'a, WordlyMessage> {
    column![
        row(
            keyboard
                .iter()
                .take(LEN_FIRST_ROW_KEYBOARD_RU)
                .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark))
        )
        .spacing(BASE_SPACE),
        row(
            keyboard
                .iter()
                .skip(LEN_FIRST_ROW_KEYBOARD_RU)
                .take(LEN_SECOND_ROW_KEYBOARD_RU)
                .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark))
        )
        .spacing(BASE_SPACE),
        row(
            keyboard
                .iter()
                .skip(LEN_FIRST_ROW_KEYBOARD_RU + LEN_SECOND_ROW_KEYBOARD_RU)
                .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark))
        )
        .spacing(BASE_SPACE),
    ]
    .spacing(BASE_SPACE + 2)
    .align_x(Alignment::Center)
    .into()
}

fn menu_button<'a>(label: &'a str, message: WordlyMessage) -> Element<'a, WordlyMessage> {
    button(text(label).size(24).style(styles::title_style))
        .padding([12, 22])
        .style(styles::menu_button_style)
        .on_press(message)
        .into()
}

fn paper_texture<'a>() -> Element<'a, WordlyMessage> {
    column![
        row![
            container(Space::new().width(Length::Fill))
                .height(6)
                .width(Length::FillPortion(5))
                .style(|_| styles::paper_highlight_style(0.8)),
            Space::new().width(18),
            container(Space::new().width(Length::Fill))
                .height(8)
                .width(Length::FillPortion(4))
                .style(|_| styles::paper_crease_style(0.6)),
        ]
        .align_y(Alignment::Center)
        .spacing(10),
        row![
            container(Space::new().width(Length::Fill))
                .height(4)
                .width(Length::FillPortion(3))
                .style(|_| styles::paper_stain_style(0.18)),
            Space::new().width(24),
            container(Space::new().width(Length::Fill))
                .height(5)
                .width(Length::FillPortion(6))
                .style(|_| styles::paper_crease_style(0.38)),
            Space::new().width(20),
            container(Space::new().width(Length::Fill))
                .height(10)
                .width(Length::FillPortion(2))
                .style(|_| styles::paper_stain_style(0.14)),
        ]
        .align_y(Alignment::Center)
        .spacing(8),
        row![
            container(Space::new().width(Length::Fill))
                .height(7)
                .width(Length::FillPortion(7))
                .style(|_| styles::torn_edge_style()),
            Space::new().width(14),
            container(Space::new().width(Length::Fill))
                .height(4)
                .width(Length::FillPortion(3))
                .style(|_| styles::paper_highlight_style(0.55)),
        ]
        .align_y(Alignment::Center)
        .spacing(10),
    ]
    .spacing(10)
    .into()
}

fn paper_block<'a>(content: impl Into<Element<'a, WordlyMessage>>) -> Element<'a, WordlyMessage> {
    container(
        column![
            paper_texture(),
            content.into(),
            paper_texture(),
        ]
        .spacing(18),
    )
        .padding([24, 28])
        .style(|_| styles::paper_panel_style())
        .into()
}

fn game_frame<'a>(content: impl Into<Element<'a, WordlyMessage>>) -> Element<'a, WordlyMessage> {
    container(
        column![
            paper_texture(),
            content.into(),
            row![
                container(Space::new().width(Length::Fill))
                    .height(6)
                    .width(Length::FillPortion(4))
                    .style(|_| styles::paper_crease_style(0.75)),
                Space::new().width(16),
                container(Space::new().width(Length::Fill))
                    .height(5)
                    .width(Length::FillPortion(3))
                    .style(|_| styles::paper_highlight_style(0.55)),
                Space::new().width(14),
                container(Space::new().width(Length::Fill))
                    .height(9)
                    .width(Length::FillPortion(2))
                    .style(|_| styles::paper_stain_style(0.16)),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
        ]
        .spacing(20),
    )
        .padding([30, 34])
        .style(|_| styles::paper_sheet_style())
        .center_x(Length::Shrink)
        .into()
}

fn header<'a>(title: &'a str, subtitle: &'a str) -> Element<'a, WordlyMessage> {
    column![
        text(title).size(42).style(styles::title_style),
        text(subtitle).size(20).style(styles::body_text_style),
    ]
    .spacing(8)
    .align_x(Alignment::Center)
    .into()
}

impl Wordly {
    pub fn view(&self) -> Element<'_, WordlyMessage> {
        match self.state {
            WordlyState::Menu => paper_block(
                column![
                    header("Wordly", "Guess the word on a rough paper sheet with ink-like feedback."),
                    menu_button("Start game", WordlyMessage::GoPlay),
                    menu_button("Back home", WordlyMessage::GoHome)
                ]
                .spacing(18)
                .align_x(Alignment::Center),
            ),
            WordlyState::InGame => game_frame(
                column![
                    header("Wordly", "A warm kraft-paper board with pressed keys and stamped marks."),
                    attempt_widget(&self.proccess_game.attempts),
                    input_attempt_widget(
                        self.proccess_game.current_input.as_str(),
                        self.proccess_game.cursor
                    ),
                    keyboard_widget(&self.proccess_game.keyboard),
                    menu_button("Back to menu", WordlyMessage::GoHome),
                ]
                .spacing(18)
                .align_x(Alignment::Center),
            ),
            WordlyState::FinishedWin => paper_block(
                column![
                    header("Solved", "The note is complete and the hidden word showed up."),
                    text(format!(
                        "You needed {} attempts. Hidden word: \"{}\"",
                        self.proccess_game.attempts.len(),
                        self.proccess_game.word
                    ))
                    .size(20)
                    .style(styles::body_text_style),
                    attempt_widget(&self.proccess_game.attempts),
                    row![
                        menu_button("Play again", WordlyMessage::GoPlay),
                        menu_button("Back to menu", WordlyMessage::GoHome)
                    ]
                    .spacing(14)
                    .align_y(Alignment::Center)
                ]
                .spacing(18)
                .align_x(Alignment::Center),
            ),
            WordlyState::FinishedLose => paper_block(
                column![
                    header("Missed", "The sheet is full, but the word never appeared."),
                    text(format!(
                        "After {} attempts the word stayed \"{}\".",
                        MAX_ATTEMPTS,
                        self.proccess_game.word
                    ))
                    .size(20)
                    .style(styles::body_text_style),
                    attempt_widget(&self.proccess_game.attempts),
                    row![
                        menu_button("Try again", WordlyMessage::GoPlay),
                        menu_button("Back to menu", WordlyMessage::GoHome)
                    ]
                    .spacing(14)
                    .align_y(Alignment::Center)
                ]
                .spacing(18)
                .align_x(Alignment::Center),
            ),
        }
    }

    pub fn key_pressed(&mut self, key_msg: KeyMessage) {
        match key_msg {
            KeyMessage::Left => self.proccess_game.move_left(),
            KeyMessage::Right => self.proccess_game.move_right(),
            KeyMessage::Char(ch) => self.proccess_game.insert_char(ch),
            KeyMessage::Enter => self.update(WordlyMessage::SubmitAttempt),
            KeyMessage::Backspace => self.proccess_game.backspace(),
        }
    }

    pub fn update(&mut self, message: WordlyMessage) {
        match message {
            WordlyMessage::GoPlay => {
                self.state = WordlyState::InGame;
                self.proccess_game = WordlyGame::new();
            }
            WordlyMessage::SubmitAttempt => {
                if self.all_worlds.is_empty() {
                    self.all_worlds = WordProvider::get_all_wards();
                }
                if self.all_worlds.contains(&self.proccess_game.current_input) {
                    self.proccess_game.update(message);
                }
                if self.proccess_game.current_input == self.proccess_game.word {
                    self.state = WordlyState::FinishedWin;
                } else if self.proccess_game.attempts.len() == MAX_ATTEMPTS {
                    self.state = WordlyState::FinishedLose;
                }
            }
            _ => {
                self.proccess_game.update(message);
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct WordlyGame {
    word: String,
    graphemes_count: usize,
    attempts: Vec<Attempt>,
    current_input: String,
    cursor: usize,
    keyboard: Vec<(String, Mark)>,
}

impl WordlyGame {
    pub fn new() -> WordlyGame {
        let all_char_ru = "\u{439}\u{446}\u{443}\u{43A}\u{435}\u{43D}\u{433}\u{448}\u{449}\u{437}\u{445}\u{44A}\u{444}\u{44B}\u{432}\u{430}\u{43F}\u{440}\u{43E}\u{43B}\u{434}\u{436}\u{44D}\u{44F}\u{447}\u{441}\u{43C}\u{438}\u{442}\u{44C}\u{431}\u{44E}";
        let mut keyboard = Vec::new();
        for i in all_char_ru.graphemes(true) {
            keyboard.push((i.to_string(), Mark::default()));
        }
        let mut current_input = String::new();
        let word = WordProvider::get_one_word_5_ru();
        let graphemes_count = word.graphemes(true).count();
        for _ in 0..graphemes_count {
            current_input.push(' ');
        }

        WordlyGame {
            word: word.clone(),
            graphemes_count,
            attempts: vec![],
            current_input,
            cursor: 0,
            keyboard,
        }
    }

    pub fn update(&mut self, message: WordlyMessage) {
        match message {
            WordlyMessage::SubmitAttempt => {
                if self.current_input.graphemes(true).count() == 5 {
                    let temp_attempt = Attempt::new(self.word.clone(), self.current_input.clone());
                    self.attempts.push(temp_attempt.clone());

                    for (i, c) in self.current_input.graphemes(true).enumerate() {
                        let status = temp_attempt.marked[i];

                        self.keyboard.iter_mut().for_each(|(sym, mark)| {
                            if sym == c {
                                *mark = match status {
                                    Mark::Present => {
                                        if *mark != Mark::Correct {
                                            Mark::Present
                                        } else {
                                            *mark
                                        }
                                    }
                                    Mark::Correct => Mark::Correct,
                                    _ => {
                                        if *mark != Mark::Present && *mark != Mark::Correct {
                                            Mark::Absent
                                        } else {
                                            *mark
                                        }
                                    }
                                };
                            }
                        });
                    }
                    self.current_input = String::new();
                    self.cursor = 0;
                    for _ in 0..self.graphemes_count {
                        self.current_input.push(' ');
                    }
                }
            }
            WordlyMessage::KeyboardClicked(sym) => {
                self.current_input =
                    replace_by_index(&mut self.current_input, self.cursor, &sym);
                if self.cursor < self.graphemes_count - 1 {
                    self.cursor += 1;
                }
            }
            _ => {}
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor < self.graphemes_count - 1 {
            self.cursor += 1;
        }
    }

    pub fn insert_char(&mut self, ch: String) {
        if ch
            .chars()
            .next()
            .is_some_and(|c| ('\u{430}'..='\u{44F}').contains(&c))
        {
            self.current_input = replace_by_index(&mut self.current_input, self.cursor, &ch);
            if self.cursor < self.graphemes_count - 1 {
                self.cursor += 1;
            }
        }
    }

    pub fn backspace(&mut self) {
        self.current_input = replace_by_index(&mut self.current_input, self.cursor, " ");
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
}

#[derive(Debug, Clone)]
pub enum WordlyMessage {
    GoHome,
    GoPlay,
    SubmitAttempt,
    KeyboardClicked(String),
}

#[derive(Debug, Clone, Default)]
pub enum WordlyState {
    #[default]
    Menu,
    InGame,
    FinishedLose,
    FinishedWin,
}
