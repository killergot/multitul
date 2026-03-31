use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use iced::{{Element, Border, Color}, widget::{button, column, text, container}, Pixels};
use iced::widget::{center_y, row, text_input, Space};
use crate::games::wordly::mark::Mark;
use super::attempt::Attempt;
use super::word_provider::WordProvider;
use super::styles;
use super::consts::*;

#[derive(Debug, Clone, Default)]
pub struct Wordly{
    state: WordlyState,
    proccess_game: WordlyGame,
    all_worlds: Vec<String>
}


fn key_widget<'a>(symbol: &'a str, mark: Mark) -> Element<'a, WordlyMessage> {
    container(text(symbol))
        .height(KEY_WIDGET_SIZE)
        .width(KEY_WIDGET_SIZE)
        .center(KEY_WIDGET_SIZE)
        .style(move |_| styles::marked_cell_style(mark))
        .into()
}

fn attempt_widget<'a>(attempts: &Vec<Attempt>) -> Element<'a, WordlyMessage> {
    column(attempts.iter().map(|attempt| {
        row(
            attempt.word.chars()
                .zip(attempt.marked)
                .map(|(char, mark)| {
                    container(text(char.to_string()))
                        .height(CHAR_WIDGET_SIZE)
                        .width(CHAR_WIDGET_SIZE)
                        .center(CHAR_WIDGET_SIZE)
                        .style(move |_| styles::marked_cell_style(mark))
                        .into()
                })
        ).into()
    })).into()
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
                ].spacing(BASE_SPACE).into()
}

impl Wordly{
    pub fn view(&self) -> Element<'_, WordlyMessage>{
        match self.state {
            WordlyState::Menu => iced::widget::column![
                    text("Wordly"),
                    button("Start game").on_press(WordlyMessage::GoPlay),
                    button("Go home").on_press(WordlyMessage::GoHome)
                ].into(),
            WordlyState::InGame => {
                column![
                    attempt_widget(&self.proccess_game.attempts),
                    text_input("пирог", &self.proccess_game.current_input)
                        .on_input(WordlyMessage::InputChanged)
                        .on_submit(WordlyMessage::SubmitAttempt)
                        .padding(TEXT_INPUT_PADDING)
                        .size(TEXT_INPUT_SIZE)
                        .width(TEXT_INPUT_WIDTH),
                    keyboard_widget(&self.proccess_game.keyboard),
                ].into()
            },
            WordlyState::FinishedWin => {
                column![
                    text("Finished"),
                    text("You win"),
                    text(format!(
                        "You spent {} attempts for word \"{}\"",
                        self.proccess_game.attempts.len(),
                        self.proccess_game.word
                    )),
                    attempt_widget(&self.proccess_game.attempts),
                    button("Go to menu").on_press(WordlyMessage::GoHome).padding([10,14])
                ].into()
            },
            WordlyState::FinishedLose => {
                column![
                    text("Finished"),
                    text("You Lose"),
                    text(format!(
                        "You spent 6 attempts for word \"{}\" and not predict!",
                        self.proccess_game.word
                    )),
                    attempt_widget(&self.proccess_game.attempts),
                    button("Main menu").on_press(WordlyMessage::GoHome).padding([10,14]),
                    button("Wordly menu").on_press(WordlyMessage::GoHome).padding([10,14])
                ].spacing(BASE_SPACE).into()
            },
            _ => iced::widget::column![].into(),
        }
    }

    pub fn update(&mut self, message: WordlyMessage){
        match message {
            WordlyMessage::GoPlay => {
                self.state = WordlyState::InGame;
                self.proccess_game = WordlyGame::new();
            },
            WordlyMessage::SubmitAttempt => {
                if self.proccess_game.current_input == self.proccess_game.word{
                    self.state = WordlyState::FinishedWin;
                    self.proccess_game.update(message);
                }
                else if self.proccess_game.attempts.len() == 5{
                    self.state = WordlyState::FinishedLose;
                }
                else{
                    if self.all_worlds.len() == 0{
                        self.all_worlds = WordProvider::get_all_wards();
                    }
                    if self.all_worlds.contains(&self.proccess_game.current_input) {
                        self.proccess_game.update(message);
                    }
                }

            }
            _ => {self.proccess_game.update(message);}
        }
    }
}


#[derive(Debug, Clone, Default)]
struct WordlyGame{
    word : String,
    attempts: Vec<Attempt>,
    current_input: String,
    // We have 4 state for any char in keyboard:
    // Absent - We know it no in the word
    // Present - We know the word contains the char, but not know where
    // Correct - We predict stead for the char
    // Unknown - We haven't some info about the char - default state
    keyboard: Vec<(String,Mark)>,
}

impl WordlyGame{
    pub fn new() -> WordlyGame{
        let all_char_ru = "йцукенгшщзхъфывапролджэячсмитьбю";
        let mut keyboard = Vec::new();
        for i in all_char_ru.graphemes(true){
            keyboard.push((i.to_string(), Mark::default()));
        }
        // WordlyGame{word:"пирог".to_string(), attempts: vec![],
        WordlyGame{word: WordProvider::get_one_word_5_ru(), attempts: vec![],
        current_input: "".to_string(),
        keyboard,}
    }
    pub fn update(&mut self, message: WordlyMessage){
        match message {
            WordlyMessage::InputChanged(txt) => {
                if txt.graphemes(true).count() <= 5 {
                    self.current_input = txt;
                }
            },
            WordlyMessage::SubmitAttempt =>{
                // create new attempt
                if self.current_input.graphemes(true).count() == 5 {
                    let temp_attempt = Attempt::new(self.word.clone(), self.current_input.clone());
                    self.attempts.push(temp_attempt.clone());

                    for (i, c) in self.current_input.graphemes(true).enumerate() {
                        let status = temp_attempt.marked[i];

                        // Используем for_each для выполнения действия
                        self.keyboard.iter_mut().for_each(|(sym, mark)| {
                            if sym == c {
                                *mark = match status {
                                    Mark::Present => if *mark != Mark::Correct { Mark::Present } else { *mark }, // На месте
                                    // Если уже 2, не меняем (приоритет точного совпадения)
                                    Mark::Correct => Mark::Correct,
                                    _ => {
                                        // Если не 1 и не 2, то 0
                                        if *mark != Mark::Present && *mark != Mark::Correct { Mark::Absent } else { *mark }
                                    }
                                };
                            }
                        });
                    }
                    self.current_input.clear();
                }
            },
            _ => {
            }
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
pub enum WordlyState{
    #[default]
    Menu,
    InGame,
    FinishedLose,
    FinishedWin
}


