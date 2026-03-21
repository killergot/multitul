mod games;
use crate::games::wordly::{Wordly, WordlyMessage};

use iced::{
    Element, Theme,
    widget::{button, column, text},
};
use iced::application::UpdateFn;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(theme)
        .run()
}

struct App {
    screen: Screen
}

impl App {
    fn new() -> Self {
        Self{
            screen: Screen::Main
        }
    }

    fn update(app: &mut Self, message: Message){
        match message {
            Message::Counter(msg) => {
                match msg {
                    CounterMessage::Increment =>
                        if let Screen::Counter(counter) = &mut app.screen {
                            counter.value += 1;
                        },
                    CounterMessage::Decrement =>
                    if let Screen::Counter(counter) = &mut app.screen {
                        counter.value -= 1;
                    }
                }
            },
            Message::Wordly(msg) => match msg {
                WordlyMessage::GoHome => {
                    app.screen = Screen::Main;
                },
                msg => if let Screen::Wordly(wordly) = &mut app.screen {
                    wordly.update(msg);
                }
            },
            Message::SwitchTo(msg) => {
                app.screen = msg;
            }
        }
    }

    fn view(app: &Self) -> Element<'_, Message> {
        match &app.screen {
            Screen::Counter(counter) => column![
                text(format!("Значение: {}", counter.value)),
                button("Увеличить").on_press(Message::Counter(CounterMessage::Increment)),
                button("Уменьшить").on_press(Message::Counter(CounterMessage::Decrement)),
                button("Go home").on_press(Message::SwitchTo(Screen::Main))
            ]
                .spacing(12)
                .padding(20)
                .into(),
            Screen::Wordly(wordly_game) => wordly_game.view().map(Message::Wordly),
            Screen::Main => column![
                text(format!("My multitul")),
                button("counter").on_press(Message::SwitchTo(Screen::Counter(Counter::default()))),
                button("wordly").on_press(Message::SwitchTo(Screen::Wordly(Wordly::default()))),
            ].spacing(12)
                .padding(20)
                .into(),
        }
    }
}
#[derive(Debug, Clone)]
enum Screen {
    Counter(Counter),
    Wordly(Wordly),
    Main
}




#[derive(Debug,Clone, Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone)]
enum Message {
    SwitchTo(Screen),
    Counter(CounterMessage),
    Wordly(WordlyMessage)
}

#[derive(Debug, Clone)]
enum CounterMessage{
    Increment,
    Decrement
}



fn theme(_app: &App) -> Theme {
    Theme::Dark
}