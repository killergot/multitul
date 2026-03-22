use iced::{
    {Element,Border,Color},
    widget::{button, column, text, container},

};
use iced::widget::row;

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
                column(
                    self.proccess_game.attempts.iter().map(|word| {
                        row(
                            word.chars()
                                .zip(self.proccess_game.word.chars())
                                .map(|(guess_char, target_char)| {
                                    let is_correct_place = guess_char == target_char;

                                    container(text(guess_char.to_string()))
                                        .height(30)
                                        .width(30)
                                        .style(move |_| {
                                            if is_correct_place {
                                                container::Style {
                                                    background: Some(Color::from_rgb(0.8, 0.8, 1.0).into()),
                                                    border: Border {
                                                        width: 2.0,
                                                        color: Color::from_rgb(0.1, 0.8, 0.3),
                                                        radius: 6.0.into(),
                                                    },
                                                    ..Default::default()
                                                }
                                            } else {
                                                Default::default()
                                            }
                                        })
                                        .into()
                                })
                        ).into()
                    })
                ).into()
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
            _ => {}
        }
    }
}




#[derive(Debug, Clone, Default)]
struct WordlyGame{
    word : String,
    step : u8,
    attempts: Vec<String>
}

impl WordlyGame{
    pub fn new() -> WordlyGame{
        WordlyGame{word: "silly".to_string(), step: 1, attempts: vec!["qwert".to_string(), "aidfg".to_string()]}
    }
}


#[derive(Debug, Clone)]
pub enum WordlyMessage {
    GoHome,
    GoPlay
}

#[derive(Debug, Clone, Default)]
pub enum WordlyState{
    #[default]
    Menu,
    InGame,
    Finished,
}