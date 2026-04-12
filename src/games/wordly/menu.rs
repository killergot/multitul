use unicode_segmentation::UnicodeSegmentation;

use super::attempt::Attempt;
use super::consts::*;
use super::styles;
use super::word_provider::WordProvider;
use crate::KeyMessage;
use crate::games::wordly::mark::Mark;
use iced::widget::row;
use iced::{
    Element, Padding,
    alignment::Horizontal,
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
            button(text("submit")).on_press(WordlyMessage::SubmitAttempt),
            button(text("<- Backspace")).on_press(WordlyMessage::BackspaceClicked),
        ]
        .padding([BASE_SPACE as u16, 0])
        .spacing(BASE_SPACE),
    ]
    .spacing(BASE_SPACE)
    .align_x(Horizontal::Center)
    .into()
}

impl Wordly {
    pub fn view(&self) -> Element<'_, WordlyMessage> {
        match self.state {
            WordlyState::Menu => column![
                text("Wordly"),
                button("Start game").on_press(WordlyMessage::GoPlay),
                button("Go home").on_press(WordlyMessage::GoHome)
            ]
            .into(),
            WordlyState::InGame => column![
                attempt_widget(&self.proccess_game.attempts),
                input_attempt_widget(
                    self.proccess_game.current_input.as_str(),
                    self.proccess_game.cursor
                ),
                keyboard_widget(&self.proccess_game.keyboard),
            ]
            .align_x(Horizontal::Center)
            .into(),
            WordlyState::FinishedWin => column![
                text("Finished"),
                text("You win"),
                text(format!(
                    "You spent {} attempts for word \"{}\"",
                    self.proccess_game.attempts.len(),
                    self.proccess_game.word
                )),
                attempt_widget(&self.proccess_game.attempts),
                button("Go to menu")
                    .on_press(WordlyMessage::GoHome)
                    .padding([10, 14])
            ]
            .into(),
            WordlyState::FinishedLose => column![
                text("Finished"),
                text("You Lose"),
                text(format!(
                    "You spent 6 attempts for word \"{}\" and not predict!",
                    self.proccess_game.word
                )),
                attempt_widget(&self.proccess_game.attempts),
                button("Main menu")
                    .on_press(WordlyMessage::GoHome)
                    .padding([10, 14]),
                button("Wordly menu")
                    .on_press(WordlyMessage::GoHome)
                    .padding([10, 14])
            ]
            .spacing(BASE_SPACE)
            .into(),
            _ => iced::widget::column![].into(),
        }
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
    // later carry this out in InputState struct
    current_input: String,
    cursor: usize,
    focused: bool,
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
        // let word = "Пирог".to_string();
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
            focused: true,
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
