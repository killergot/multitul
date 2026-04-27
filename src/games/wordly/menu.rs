use unicode_segmentation::UnicodeSegmentation;

use super::attempt::Attempt;
use super::consts::*;
use super::styles;
use super::word_provider::WordProvider;
use crate::KeyMessage;
use crate::games::wordly::mark::Mark;
use crate::utils::style::{self, DISPLAY_FONT, TEXT_DIM};
use iced::widget::row;
use iced::{
    Element, Length, Padding,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, text},
};

#[derive(Debug, Clone, Default)]
pub struct Wordly {
    state: WordlyState,
    proccess_game: WordlyGame,
    all_worlds: Vec<String>,
}

fn key_widget<'a>(symbol: &'a str, mark: Mark) -> Element<'a, WordlyMessage> {
    button(text(symbol))
        .height(KEY_WIDGET_SIZE)
        .width(KEY_WIDGET_SIZE)
        .style(move |theme, status| styles::keyboard_button_style(theme, status, mark))
        .on_press(WordlyMessage::KeyboardSymbolClicked(symbol.to_string()))
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

fn create_rows_char<'a>(
    items: impl Iterator<Item = (&'a str, Mark)>,
) -> Element<'a, WordlyMessage> {
    row(items.map(|(char, mark)| {
        container(text(char))
            .height(CHAR_WIDGET_SIZE)
            .width(CHAR_WIDGET_SIZE)
            .center(CHAR_WIDGET_SIZE)
            .style(move |_| styles::marked_cell_style(mark))
            .into()
    }))
    .spacing(BASE_SPACE)
    .padding(Padding {
        bottom: BASE_SPACE as f32,
        ..Default::default()
    })
    .into()
}

fn attempt_widget<'a>(attempts: &'a Vec<Attempt>) -> Element<'a, WordlyMessage> {
    column(
        attempts
            .iter()
            .map(|attempt| create_rows_char(attempt.word.graphemes(true).zip(attempt.marked))),
    )
    .into()
}

fn input_attempt_widget<'a>(input_text: &'a str, cursor: usize) -> Element<'a, WordlyMessage> {
    let mapped = input_text.graphemes(true).enumerate().map(|(i, sym)| {
        let mark = if i == cursor {
            Mark::Cursor
        } else {
            Mark::Unknown
        };

        (sym, mark)
    });

    create_rows_char(mapped)
}

fn keyboard_widget<'a>(keyboard: &'a Vec<(String, Mark)>) -> Element<'a, WordlyMessage> {
    column![
        row(keyboard
            .iter()
            .take(LEN_FIRST_ROW_KEYBOARD_RU)
            .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark)))
        .spacing(BASE_SPACE),
        row(keyboard
            .iter()
            .skip(LEN_FIRST_ROW_KEYBOARD_RU)
            .take(LEN_SECOND_ROW_KEYBOARD_RU)
            .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark)))
        .spacing(BASE_SPACE),
        row(keyboard
            .iter()
            .skip(LEN_FIRST_ROW_KEYBOARD_RU + LEN_SECOND_ROW_KEYBOARD_RU)
            .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark)))
        .spacing(BASE_SPACE),
        row![
            button(text("⌫ Стереть").size(13).center())
                .on_press(WordlyMessage::BackspaceClicked)
                .padding(Padding::from([8, 14]))
                .style(styles::ghost_button),
            button(text("Подтвердить").size(13).center())
                .on_press(WordlyMessage::SubmitAttempt)
                .padding(Padding::from([8, 14]))
                .style(styles::primary_button),
        ]
        .padding([BASE_SPACE as u16, 0])
        .spacing(BASE_SPACE),
    ]
    .spacing(BASE_SPACE)
    .align_x(Horizontal::Center)
    .into()
}

fn wordly_menu_button(label: &str, message: WordlyMessage) -> button::Button<'_, WordlyMessage> {
    button(text(label).size(15).center())
        .on_press(message)
        .padding(Padding::from([12, 18]))
        .width(Length::Fill)
        .style(styles::ghost_button)
}

impl Wordly {
    pub fn view(&self) -> Element<'_, WordlyMessage> {
        match self.state {
            WordlyState::Menu => self.menu_screen("WORDLY", "Угадайте слово из пяти букв"),
            WordlyState::InGame => container(
                column![
                    attempt_widget(&self.proccess_game.attempts),
                    input_attempt_widget(
                        self.proccess_game.current_input.as_str(),
                        self.proccess_game.cursor
                    ),
                    keyboard_widget(&self.proccess_game.keyboard),
                ]
                .spacing(BASE_SPACE)
                .align_x(Horizontal::Center),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(24)
            .into(),
            WordlyState::FinishedWin => self.finished_screen(
                "Победа",
                format!(
                    "Слово «{}» взято за {} попыток.",
                    self.proccess_game.word,
                    self.proccess_game.attempts.len()
                ),
            ),
            WordlyState::FinishedLose => self.finished_screen(
                "Поражение",
                format!(
                    "Слово было «{}». Шесть попыток исчерпаны.",
                    self.proccess_game.word
                ),
            ),
        }
    }

    fn menu_screen<'a>(&'a self, title: &'a str, subtitle: &'a str) -> Element<'a, WordlyMessage> {
        let panel = container(
            column![
                container(text("").width(Length::Fixed(56.0)).height(Length::Fixed(3.0)))
                    .style(style::accent_strip),
                text(title).font(DISPLAY_FONT).size(48),
                text(subtitle)
                    .size(14)
                    .style(|_| iced::widget::text::Style { color: Some(TEXT_DIM) }),
                container(text("")).height(12),
                wordly_menu_button("01 · Начать партию", WordlyMessage::GoPlay),
                wordly_menu_button("02 · В главное меню", WordlyMessage::GoHome),
            ]
            .spacing(12)
            .align_x(Horizontal::Center),
        )
        .padding(Padding::from([28, 32]))
        .width(Length::Fixed(360.0))
        .style(styles::menu_panel);

        container(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(24)
            .into()
    }

    fn finished_screen<'a>(
        &'a self,
        title: &'a str,
        subtitle: String,
    ) -> Element<'a, WordlyMessage> {
        let panel = container(
            column![
                container(text("").width(Length::Fixed(56.0)).height(Length::Fixed(3.0)))
                    .style(style::accent_strip),
                text(title).font(DISPLAY_FONT).size(40),
                text(subtitle)
                    .size(14)
                    .style(|_| iced::widget::text::Style { color: Some(TEXT_DIM) }),
                container(attempt_widget(&self.proccess_game.attempts))
                    .center_x(Length::Fill),
                container(text("")).height(8),
                wordly_menu_button("01 · Снова в бой", WordlyMessage::GoPlay),
                wordly_menu_button("02 · В главное меню", WordlyMessage::GoHome),
            ]
            .spacing(12)
            .align_x(Horizontal::Center),
        )
        .padding(Padding::from([28, 32]))
        .width(Length::Fixed(420.0))
        .style(styles::menu_panel);

        container(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(24)
            .into()
    }

    pub fn key_pressed(&mut self, key_msg: KeyMessage) {
        match key_msg {
            KeyMessage::Left => self.proccess_game.move_left(),
            KeyMessage::Right => self.proccess_game.move_right(),
            KeyMessage::Char(ch) => self.proccess_game.insert_char(ch),
            KeyMessage::Enter => self.update(WordlyMessage::SubmitAttempt),
            KeyMessage::Backspace => self.proccess_game.backspace(),
            _ => {}
        }
    }

    pub fn update(&mut self, message: WordlyMessage) {
        match message {
            WordlyMessage::GoPlay => {
                self.state = WordlyState::InGame;
                self.proccess_game = WordlyGame::new();
            }
            WordlyMessage::SubmitAttempt => {
                if self.all_worlds.len() == 0 {
                    self.all_worlds = WordProvider::get_all_wards();
                }
                if self.all_worlds.contains(&self.proccess_game.current_input) {
                    let is_win = self.proccess_game.current_input == self.proccess_game.word;
                    self.proccess_game.update(message);
                    if is_win {
                        self.state = WordlyState::FinishedWin;
                    } else if self.proccess_game.attempts.len() == MAX_ATTEMPTS {
                        self.state = WordlyState::FinishedLose;
                    }
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
    // later carry this out in InputState struct
    current_input: String,
    cursor: usize,
    // We have 4 state for any char in keyboard:
    // Absent - We know it no in the word
    // Present - We know the word contains the char, but not know where
    // Correct - We predict stead for the char
    // Unknown - We haven't some info about the char - default state
    keyboard: Vec<(String, Mark)>,
}

impl WordlyGame {
    pub fn new() -> WordlyGame {
        let all_char_ru = "йцукенгшщзхъфывапролджэячсмитьбю";
        let mut keyboard = Vec::new();
        for i in all_char_ru.graphemes(true) {
            keyboard.push((i.to_string(), Mark::default()));
        }
        let mut current_input = String::new();
        // let word = "пирог".to_string();
        let word = WordProvider::get_one_word_5_ru();
        let graphemes_count = word.graphemes(true).count();
        for i in 0..graphemes_count {
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
                // create new attempt
                if self.current_input.graphemes(true).count() == 5 {
                    let temp_attempt = Attempt::new(self.word.clone(), self.current_input.clone());
                    self.attempts.push(temp_attempt.clone());

                    for (i, c) in self.current_input.graphemes(true).enumerate() {
                        let status = temp_attempt.marked[i];

                        // Используем for_each для выполнения действия
                        // map не подходит, ибо нужен собиратель
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
            WordlyMessage::KeyboardSymbolClicked(sym) => {
                self.current_input = replace_by_index(&mut self.current_input, self.cursor, &sym);
                if self.cursor < self.graphemes_count - 1 {
                    self.cursor += 1;
                }
            }
            WordlyMessage::BackspaceClicked => {
                self.backspace();
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
        if ('а'..='я').contains(&ch.chars().next().unwrap()) {
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
    BackspaceClicked,
    KeyboardSymbolClicked(String),
}

#[derive(Debug, Clone, Default)]
pub enum WordlyState {
    #[default]
    Menu,
    InGame,
    FinishedLose,
    FinishedWin,
}
