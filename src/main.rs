mod games;
mod core;
mod utils;
mod main_test;

use crate::games::wordly::{Wordly, WordlyMessage};

use iced::{Element, Theme, widget::{button, column, text}, Length};
use iced::widget::{container, stack};
use iced::{event, keyboard, Event, Subscription};
use iced::keyboard::key::Named;
use iced::keyboard::Key;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(theme)
        .subscription(App::subscription)
        .run()
}

struct App {
    screen: Screen
}

fn sign_widget<'a>() -> Element<'a, Message> {
    column![
        text("Multitul"),
        text("by @rubi_ck")
    ].into()
}

impl App {
    fn new() -> Self {
        Self{
            screen: Screen::Main
        }
    }

    fn subscription(_app: &Self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                                    key,
                                    text,
                                    ..
                                }) => {

                    match key.as_ref() {
                        Key::Named(Named::ArrowLeft) => {
                            Some(Message::KeyPressed(KeyMessage::Left))
                        }
                        Key::Named(Named::ArrowRight) => {
                            Some(Message::KeyPressed(KeyMessage::Right))
                        }
                        Key::Named(Named::Backspace) => {
                            Some(Message::KeyPressed(KeyMessage::Backspace))
                        }
                        Key::Named(Named::Enter) => {
                            Some(Message::KeyPressed(KeyMessage::Enter))
                        }
                        _ => {
                            text.map(|t| Message::KeyPressed(KeyMessage::Char(t.to_string())))
                        }
                    }
                }
                _ => None,
            }
        })
    }

    fn update(app: &mut Self, message: Message){
        match message {
            Message::KeyPressed(key_msg) => {
                if let Screen::Wordly(wordly) = &mut app.screen {
                    wordly.key_pressed(key_msg);
                }
            },
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
        let content = match &app.screen {
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
        };
        stack![container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
            sign_widget()
        ].into()
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
    Wordly(WordlyMessage),
    KeyPressed(KeyMessage)
}

#[derive(Debug, Clone)]
enum KeyMessage {
    Left,
    Right,
    Backspace,
    Enter,
    Char(String),
}

#[derive(Debug, Clone)]
enum CounterMessage{
    Increment,
    Decrement
}



fn theme(_app: &App) -> Theme {
    Theme::Dark
}