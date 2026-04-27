mod core;
mod games;
mod macros;
mod utils;

use std::time::Duration;
use iced::time::every;
use crate::games::wordly::{Wordly, WordlyMessage};

use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::widget::{container, mouse_area, row, scrollable, stack, svg};
use iced::{Background, Border, Color, Element, Length, Theme, widget::{button, column, text}, Task};
use iced::{Event, Subscription, event, keyboard, mouse};

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
use crate::games::one_brain::menu::{Brain, BrainMessage};

const SPLITTER_HEIGHT: f32 = 2.0;
const BOTTOM_PANEL_DEFAULT: f32 = 250.0;
const BOTTOM_PANEL_MIN: f32 = 80.0;
const BOTTOM_PANEL_MAX: f32 = 800.0;

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
    bottom_panel_height: f32,
    cursor_y: f32,
    drag_anchor: Option<(f32, f32)>,
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

        let network = Network::new(None);

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
            bottom_panel_height: BOTTOM_PANEL_DEFAULT,
            cursor_y: 0.0,
            drag_anchor: None,
        }
    }

    fn subscription(app: &Self) -> Subscription<Message> {
        let brain_sub = match &app.screen {
            Screen::Brain(brain) => brain.subscription().map(Message::Brain),
            _ => Subscription::none(),
        };

        Subscription::batch([
            event::listen_with(|event, _status, _window| match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, text, .. }) => match key.as_ref() {
                    Key::Named(Named::ArrowLeft) => Some(Message::KeyPressed(KeyMessage::Left)),
                    Key::Named(Named::ArrowRight) => Some(Message::KeyPressed(KeyMessage::Right)),
                    Key::Named(Named::Backspace) => Some(Message::KeyPressed(KeyMessage::Backspace)),
                    Key::Named(Named::Enter) => Some(Message::KeyPressed(KeyMessage::Enter)),
                    _ => text.map(|t| Message::KeyPressed(KeyMessage::Char(t.to_string()))),
                },
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    Some(Message::CursorMoved(position.y))
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    Some(Message::SplitDragEnd)
                }
                _ => None,
            }),
            every(Duration::from_secs(1)).map(|_| Message::NetworkTick),
            brain_sub
        ])
    }

    fn update(app: &mut Self, message: Message) -> Task<Message> {
        match message {
            Message::NetworkTick => {
                Task::perform(NetworkState::check_network(), Message::NetworkChecked)
            }
            Message::NetworkChecked(state) => {
                app.network.set_state(state);
                Task::none()
            }
            Message::KeyPressed(key_msg) => {
                if let Screen::Wordly(wordly) = &mut app.screen {
                    wordly.key_pressed(key_msg);
                }
                Task::none()

            }
            Message::Counter(msg) => match msg {
                CounterMessage::Increment => {
                    if let Screen::Counter(counter) = &mut app.screen {
                        counter.value += 1;
                    }
                    Task::none()

                }
                CounterMessage::Decrement => {
                    if let Screen::Counter(counter) = &mut app.screen {
                        counter.value -= 1;
                    }
                    Task::none()

                }
            },
            Message::Wordly(msg) => match msg {
                WordlyMessage::GoHome => {
                    app.screen = Screen::Main;
                    Task::none()
                }
                msg => {
                    if let Screen::Wordly(wordly) = &mut app.screen {
                        wordly.update(msg);
                    }
                    Task::none()

                }
            },
            Message::Brain(msg) => match msg {
                BrainMessage::GoHome => {
                    app.screen = Screen::Main;
                    Task::none()
                }
                msg =>{
                    if let Screen::Brain(brain) = &mut app.screen {
                        brain.update(msg).map(Message::Brain)
                    } else {
                        Task::none()
                    }
                }
            }
            Message::SwitchTo(msg) => {
                app.screen = msg;
                Task::none()
            }
            Message::CursorMoved(y) => {
                app.cursor_y = y;
                if let Some((anchor_y, anchor_h)) = app.drag_anchor {
                    let new_h = anchor_h + (anchor_y - y);
                    app.bottom_panel_height = new_h.clamp(BOTTOM_PANEL_MIN, BOTTOM_PANEL_MAX);
                }
                Task::none()
            }
            Message::SplitDragStart => {
                app.drag_anchor = Some((app.cursor_y, app.bottom_panel_height));
                Task::none()
            }
            Message::SplitDragEnd => {
                app.drag_anchor = None;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match &self.screen {
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
            Screen::Brain(brain) => brain.view().map(Message::Brain),
            Screen::Main => column![
                text(format!("My multitul")),
                button("counter").on_press(Message::SwitchTo(Screen::Counter(Counter::default()))),
                button("wordly").on_press(Message::SwitchTo(Screen::Wordly(Wordly::default()))),
                button("one brain").on_press(Message::SwitchTo(Screen::Brain(Brain::default())))
            ]
            .spacing(12)
            .padding(20)
            .into(),
        };

        let top_area = container(content)
            .width(Length::Fill)
            .height(Length::Fill);

        let splitter = mouse_area(
            container(text(""))
                .width(Length::Fill)
                .height(SPLITTER_HEIGHT)
                .style(splitter_track_style),
        )
        .interaction(mouse::Interaction::ResizingVertically)
        .on_press(Message::SplitDragStart);

        let bottom_panel = container(
            row![
                container(
                    scrollable(git_widget(
                        &self.git_state.layout,
                        &self.git_edge_cache,
                        &self.git_node_cache,
                    ))
                    .width(Length::Fill)
                    .height(Length::Fill),
                )
                .width(Length::FillPortion(3))
                .height(Length::Fill)
                .padding(12),
                container(sign_widget())
                    .width(Length::FillPortion(1))
                    .height(Length::Fill)
                    .padding(12)
                    .align_x(iced::alignment::Horizontal::Right)
                    .align_y(iced::alignment::Vertical::Bottom),
            ]
            .spacing(0)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fixed(self.bottom_panel_height))
        .style(bottom_dock_style);

        let main_column = column![top_area, splitter, bottom_panel]
            .width(Length::Fill)
            .height(Length::Fill);

        stack![
            main_column,
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

fn splitter_track_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.86, 0.50, 0.27, 0.45))),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

fn bottom_dock_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.08, 0.10, 0.14))),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

#[derive(Debug, Clone)]
enum Screen {
    Counter(Counter),
    Wordly(Wordly),
    Brain(Brain),
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
    Brain(BrainMessage),
    KeyPressed(KeyMessage),
    NetworkTick,
    NetworkChecked(Option<NetworkState>),
    CursorMoved(f32),
    SplitDragStart,
    SplitDragEnd,
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
