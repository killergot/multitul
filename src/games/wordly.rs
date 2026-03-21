use iced::{
    Element,
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
                            word.chars().map(|c| text(c.to_string()).into())
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
        WordlyGame{word: "silly".to_string(), step: 1, attempts: vec!["qwert".to_string(), "asdfg".to_string()]}
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