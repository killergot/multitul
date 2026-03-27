use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use iced::{
    {Element,Border,Color},
    widget::{button, column, text, container},

};
use iced::widget::{center_y, row, text_input};


use super::attempt::Attempt;
use super::word_provider::WordProvider;

#[derive(Debug, Clone, Default)]
pub struct Wordly{
    state: WordlyState,
    proccess_game: WordlyGame,
    all_worlds: Vec<String>
}


fn key_widget<'a>(symbol: &'a str, mark: u8) -> Element<'a, WordlyMessage> {
    container(text(symbol))
        .height(30)
        .width(30)
        .center(30)
        .style(move |_| {
            if mark == 2 {
                container::Style {
                    background: Some(Color::from_rgb(0.4, 0.0, 0.4).into()),
                    border: Border {
                        width: 2.0,
                        color: Color::from_rgb(0.1, 0.8, 0.3),
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }
            } else if mark == 1 {
                container::Style {
                    background: Some(Color::from_rgb(0.0, 0.0, 0.2).into()),
                    border: Border {
                        width: 2.0,
                        color: Color::from_rgb(0.0, 0.8, 0.0),
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }
            } else if mark == 0 {
                container::Style {
                    background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
                    border: Border {
                        width: 2.0,
                        color: Color::from_rgb(0.0, 0.0, 0.0),
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }
            } else {
                Default::default()
            }
        })
        .into()
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
                let temp = column(self.proccess_game.attempts.iter().map(|attempt| {
                    row(
                        attempt.word.chars()
                            .zip(attempt.marked)
                            .map(|(char, mark)| {

                                container(text(char.to_string()))
                                    .height(60)
                                    .width(60)
                                    .center(60)
                                    .style(move |_| {
                                        if mark == 2 {
                                            container::Style {
                                                background: Some(Color::from_rgb(0.4, 0.0, 0.4).into()),
                                                border: Border {
                                                    width: 2.0,
                                                    color: Color::from_rgb(0.1, 0.8, 0.3),
                                                    radius: 6.0.into(),
                                                },
                                                ..Default::default()
                                            }
                                        } else if mark == 1 {
                                            container::Style {
                                                background: Some(Color::from_rgb(0.0, 0.0, 0.2).into()),
                                                border: Border {
                                                    width: 2.0,
                                                    color: Color::from_rgb(0.0, 0.8, 0.0),
                                                    radius: 6.0.into(),
                                                },
                                                ..Default::default()
                                            }
                                        }
                                        else {
                                            Default::default()
                                        }
                                    })
                                    .into()
                            })
                    ).into()
                }));
                let keyboard = &self.proccess_game.keyboard;

                let temp2 = column![
                    row(
                        keyboard
                            .iter()
                            .take(12)
                            .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark))
                    )
                    .spacing(5),

                    row(
                        keyboard
                            .iter()
                            .skip(12)
                            .take(11)
                            .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark))
                    )
                    .spacing(5),

                    row(
                        keyboard
                            .iter()
                            .skip(23)
                            .map(|(symbol, mark)| key_widget(symbol.as_str(), *mark))
                    )
                    .spacing(5),
                ].spacing(5);

                column![
                    temp,
                    text_input("пирог", &self.proccess_game.current_input)
                        .on_input(WordlyMessage::InputChanged)
                        .on_submit(WordlyMessage::SubmitAttempt)
                        .padding(10)
                        .size(16)
                        .width(300),
                    temp2
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
                    button("Main menu").on_press(WordlyMessage::GoHome).padding([10,14]),
                    button("Wordly menu").on_press(WordlyMessage::GoHome).padding([10,14])
                ].into()
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
    // 0 - We know it no in the word
    // 1 - We know the word contains the char, but not know where
    // 2 - We predict stead for the char
    // 3 - We haven't some info about the char - initional state
    keyboard: Vec<(String,u8)>,
}

impl WordlyGame{
    pub fn new() -> WordlyGame{
        let all_char_ru = "йцукенгшщзхъфывапролджэячсмитьбю";
        let mut keyboard = Vec::new();
        for i in all_char_ru.graphemes(true){
            keyboard.push((i.to_string(), 3));
        }
        WordlyGame{word:"пирог".to_string(), attempts: vec![],
        // WordlyGame{word: WordProvider::get_one_word_5_ru(), attempts: vec![],
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
                        print!("status =  {}, mark = ",status);

                        // Используем for_each для выполнения действия
                        self.keyboard.iter_mut().for_each(|(sym, mark)| {
                            if sym == c {
                                *mark = match status {
                                    1 => if *mark != 2 { 1 } else { *mark }, // На месте
                                    // Если уже 2, не меняем (приоритет точного совпадения)
                                    2 => 2,
                                    _ => {
                                        // Если не 1 и не 2, то 0
                                        if *mark != 1 && *mark != 2 { 0 } else { *mark }
                                    }
                                };
                                println!("{mark}");
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


