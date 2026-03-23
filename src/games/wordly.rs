use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use iced::{
    {Element,Border,Color},
    widget::{button, column, text, container},

};
use iced::widget::{center_y, row, text_input};

#[derive(Debug, Clone, Default)]
pub struct Wordly{
    state: WordlyState,
    proccess_game: WordlyGame
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
                                        }
                                        else {
                                            Default::default()
                                        }
                                    })
                                    .into()
                            })
                    ).into()
                }));
                column![
                    temp,
                    text_input("пирог", &self.proccess_game.current_input)
                        .on_input(WordlyMessage::InputChanged)
                        .on_submit(WordlyMessage::SubmitAttempt)
                        .padding(10)
                        .size(16)
                        .width(200)
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
            }
            _ => {self.proccess_game.update(message);}
        }
    }
}




#[derive(Debug, Clone, Default)]
struct WordlyGame{
    word : String,
    step : u8,
    attempts: Vec<Attempt>,
    current_input: String,
}

impl WordlyGame{
    pub fn new() -> WordlyGame{
        WordlyGame{word: "silly".to_string(), step: 1, attempts: vec![
            Attempt::new("silly".to_string(),"qwert".to_string()),
            Attempt::new("silly".to_string(),"aighl".to_string()),
            Attempt::new("silly".to_string(),"lilil".to_string())],
        current_input: "".to_string(),}
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
                    self.attempts.push(Attempt::new("silly".to_string(), self.current_input.clone()));
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
    Finished,
}


#[derive(Debug, Clone, Default)]
pub struct Attempt{
    word: String,
    // 0 - nothing, 1 - somewhere, 2 - in point
    // exaple: goal - ship, attempt - glip => marked - [2,1,0,0]
    marked: [u8; 5]
}

impl Attempt{
    pub fn new(goal_word: String, attempt_word: String) -> Attempt{
        let mut counter: HashMap<char, u8> = HashMap::new();
        let mut marked = [0;5];
        for c in goal_word.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        for (i,c) in attempt_word.chars().enumerate() {
            if goal_word.chars().nth(i) == Some(c){
                marked[i] = 2;
                *counter.get_mut(&c).unwrap() -= 1;
            }
        }
        for (i,c) in attempt_word.chars().enumerate() {
            if counter.contains_key(&c) && counter[&c] > 0 && marked[i] != 2{
                marked[i] = 1;
                *counter.get_mut(&c).unwrap() -= 1;
            }
        }

        Attempt{word: attempt_word, marked}
    }
}