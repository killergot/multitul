mod core;
mod games;
mod macros;
mod utils;

use crate::games::wordly::{Wordly, WordlyMessage};

use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::widget::{container, scrollable, stack, svg};
use iced::{
    Element, Length, Theme,
    widget::{button, column, text},
};
use iced::{Event, Subscription, event, keyboard};

use crate::core::git::widget::git_widget;
use crate::core::sign::sign_widget;
use crate::utils::git::GitGraph;
use crate::utils::git::GitStorage;
use crate::utils::git::graph_layout::GraphLayout;
use crate::utils::git::provider::GitProvider;
use crate::utils::git::state::GitState;
use iced::widget::canvas::Cache;
use crate::core::network::network::Network;
use crate::core::network::state::NetworkState;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(theme)
        .subscription(App::subscription)
        .run()
}

struct App {
    screen: Screen,
    network: Network,
    git_state: GitState,
    git_edge_cache: Cache,
    git_node_cache: Cache,
}

impl App {
    fn new() -> Self {
        let mut provider = GitProvider::new();
        match provider.scan_repository() {
            Err(e) => panic!("{}", e),
            _ => {}
        }

        let graph = GitGraph::new(&provider.repository.commits);
        let ordered_nodes = graph.topo_for_layout(&provider.repository);
        let layout: GraphLayout = GraphLayout::new(&ordered_nodes);

        let network = Network::new(NetworkState::check_network());

        Self {
            screen: Screen::Main,
            git_state: GitState {
                graph,
                repo: provider.repository.clone(),
                layout,
            },
            network,
            git_edge_cache: Cache::new(),
            git_node_cache: Cache::new(),
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

    fn view(&self) -> Element<'_, Message> {
        let content = match &self.screen {
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
            ]
            .spacing(12)
            .padding(20)
            .into(),
        };
        stack![
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            container(sign_widget())
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Right)
                .align_y(iced::alignment::Vertical::Bottom)
                .padding(20),
            container(
                scrollable(git_widget(
                    &self.git_state.layout,
                    &self.git_edge_cache,
                    &self.git_node_cache
                ))
                .height(220)
                .width(320)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Bottom)
            .padding(20),
            container(
                svg(self.network.get_icon().clone())
                      .width(24)
                      .height(24)
              )
              .width(Length::Fill)
              .height(Length::Fill)
              .align_x(iced::alignment::Horizontal::Right)
              .align_y(iced::alignment::Vertical::Top)
              .padding(20),
        ]
        .into()
    }
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
    Theme::Dark
}
