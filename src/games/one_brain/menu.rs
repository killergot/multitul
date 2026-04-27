use iced::Element;
use iced::widget::{column, Button};
use iced::widget::button;
use crate::games::wordly::WordlyMessage;

#[derive(Debug,Clone,Default)]
pub struct Brain{
    pub state: BrainState,
    pub room: Option<String>,
    pub attempts: Vec<String>,
    pub user_name: Option<String>,
    pub chat: Vec<String>,
}

impl Brain{
    pub fn view(&self) -> Element<'_, BrainMessage> {
        match self.state {
            BrainState::Menu => {
                column![
                    button("Зайти в игру"),
                    button("Посмотреть список комнат"),
                    button("Назад").on_press(BrainMessage::GoHome)
                ].spacing(10).into()
            },
            _ => column![].into(),
        }
    }

    pub fn update(&mut self, msg: BrainMessage) {
        match msg {

            _ => {}
        }
    }
}

#[derive(Default,Clone,Debug)]
pub enum BrainState {
    #[default]
    Menu,
    InRoom,
    SelectRoom,
    WrongRoom,
    FinishedGame
}

#[derive(Clone, Debug)]
pub enum BrainMessage {
    GoHome,
    SendWord(String),
    SendMessage(String),
    LeaveRoom
}