mod games;
use crate::games::wordly::{styles, Wordly, WordlyMessage};

use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::widget::{Space, button, column, container, row, text};
use iced::{event, keyboard, Alignment, Element, Event, Length, Subscription, Theme};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(theme)
        .subscription(App::subscription)
        .run()
}

struct App {
    screen: Screen,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Main,
        }
    }

    fn subscription(_app: &Self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, text, .. }) => match key.as_ref() {
                Key::Named(Named::ArrowLeft) => Some(Message::KeyPressed(KeyMessage::Left)),
                Key::Named(Named::ArrowRight) => Some(Message::KeyPressed(KeyMessage::Right)),
                Key::Named(Named::Backspace) => Some(Message::KeyPressed(KeyMessage::Backspace)),
                Key::Named(Named::Enter) => Some(Message::KeyPressed(KeyMessage::Enter)),
                _ => text.map(|t| Message::KeyPressed(KeyMessage::Char(t.to_string()))),
            },
            _ => None,
        })
    }

    fn update(app: &mut Self, message: Message) {
        match message {
            Message::KeyPressed(key_msg) => {
                if let Screen::Wordly(wordly) = &mut app.screen {
                    wordly.key_pressed(key_msg);
                }
            }
            Message::Counter(msg) => match msg {
                CounterMessage::Increment => {
                    if let Screen::Counter(counter) = &mut app.screen {
                        counter.value += 1;
                    }
                }
                CounterMessage::Decrement => {
                    if let Screen::Counter(counter) = &mut app.screen {
                        counter.value -= 1;
                    }
                }
            },
            Message::Wordly(msg) => match msg {
                WordlyMessage::GoHome => {
                    app.screen = Screen::Main;
                }
                msg => {
                    if let Screen::Wordly(wordly) = &mut app.screen {
                        wordly.update(msg);
                    }
                }
            },
            Message::SwitchTo(msg) => {
                app.screen = msg;
            }
        }
    }

    fn view(app: &Self) -> Element<'_, Message> {
        let content = match &app.screen {
            Screen::Counter(counter) => paper_card(
                column![
                    text(format!("Value: {}", counter.value))
                        .size(34)
                        .style(styles::title_style),
                    text("A small utility shown on a paper card.")
                        .size(18)
                        .style(styles::body_text_style),
                    button(text("Increase").size(22).style(styles::title_style))
                        .style(styles::menu_button_style)
                        .padding([10, 18])
                        .on_press(Message::Counter(CounterMessage::Increment)),
                    button(text("Decrease").size(22).style(styles::title_style))
                        .style(styles::menu_button_style)
                        .padding([10, 18])
                        .on_press(Message::Counter(CounterMessage::Decrement)),
                    button(text("Back home").size(22).style(styles::title_style))
                        .style(styles::menu_button_style)
                        .padding([10, 18])
                        .on_press(Message::SwitchTo(Screen::Main)),
                ]
                .spacing(14)
                .align_x(Alignment::Center),
            ),
            Screen::Wordly(wordly_game) => wordly_game.view().map(Message::Wordly),
            Screen::Main => paper_card(
                column![
                    text("Rust Multitul").size(48).style(styles::title_style),
                    text("A set of small tools wrapped in warm kraft paper and ink-like contrast.")
                        .size(20)
                        .style(styles::body_text_style),
                    button(text("Counter").size(24).style(styles::title_style))
                        .style(styles::menu_button_style)
                        .padding([12, 22])
                        .on_press(Message::SwitchTo(Screen::Counter(Counter::default()))),
                    button(text("Wordly").size(24).style(styles::title_style))
                        .style(styles::menu_button_style)
                        .padding([12, 22])
                        .on_press(Message::SwitchTo(Screen::Wordly(Wordly::default()))),
                ]
                .spacing(18)
                .align_x(Alignment::Center),
            ),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(24)
            .style(|_| styles::desk_background_style())
            .into()
    }
}

fn paper_texture<'a>() -> Element<'a, Message> {
    column![
        row![
            container(Space::new().width(Length::Fill))
                .width(Length::FillPortion(5))
                .height(6)
                .style(|_| styles::paper_highlight_style(0.8)),
            Space::new().width(16),
            container(Space::new().width(Length::Fill))
                .width(Length::FillPortion(4))
                .height(8)
                .style(|_| styles::paper_crease_style(0.65)),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        row![
            container(Space::new().width(Length::Fill))
                .width(Length::FillPortion(2))
                .height(8)
                .style(|_| styles::paper_stain_style(0.14)),
            Space::new().width(18),
            container(Space::new().width(Length::Fill))
                .width(Length::FillPortion(6))
                .height(5)
                .style(|_| styles::paper_crease_style(0.42)),
            Space::new().width(14),
            container(Space::new().width(Length::Fill))
                .width(Length::FillPortion(3))
                .height(6)
                .style(|_| styles::torn_edge_style()),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    ]
    .spacing(10)
    .into()
}

fn paper_card<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(
        column![
            paper_texture(),
            content.into(),
            paper_texture(),
        ]
        .spacing(18)
        .align_x(Alignment::Center),
    )
    .padding([28, 34])
    .style(|_| styles::paper_panel_style())
    .into()
}

#[derive(Debug, Clone)]
enum Screen {
    Counter(Counter),
    Wordly(Wordly),
    Main,
}

#[derive(Debug, Clone, Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone)]
enum Message {
    SwitchTo(Screen),
    Counter(CounterMessage),
    Wordly(WordlyMessage),
    KeyPressed(KeyMessage),
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
enum CounterMessage {
    Increment,
    Decrement,
}

fn theme(_app: &App) -> Theme {
    Theme::Light
}
