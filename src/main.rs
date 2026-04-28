mod core;
mod games;
mod macros;
mod utils;

use std::time::Duration;
use iced::time::every;
use crate::games::wordly::{Wordly, WordlyMessage};

use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::widget::{container, mouse_area, row, scrollable, stack, svg};
use iced::{Background, Border, Color, Element, Length, Padding, Theme, widget::{button, column, text}, Task};
use iced::{Event, Subscription, event, keyboard, mouse};

use crate::utils::style::{
    self as design, ACCENT, BODY_FONT, DISPLAY_FONT, DIVIDER, SURFACE_1, TEXT_DIM,
};

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

const SPLITTER_HEIGHT: f32 = 5.0;
const BOTTOM_PANEL_DEFAULT: f32 = 250.0;
const BOTTOM_PANEL_MIN: f32 = 80.0;
const BOTTOM_PANEL_MAX: f32 = 800.0;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(theme)
        .default_font(BODY_FONT)
        .subscription(App::subscription)
        .run()
}

struct App {
    screen: Screen,
    network: Network,

    git_state: Option<GitState>,
    git_edge_cache: Cache,
    git_node_cache: Cache,

    bottom_panel_height: f32,
    cursor_y: f32,
    drag_anchor: Option<(f32, f32)>,
}

impl App {
    fn new() -> Self {
        let mut provider = GitProvider::new();
        let git_state = match provider.scan_repository() {
            Err(e) => None,
            _ => {
                let graph = GitGraph::new(&provider.repository.commits);
                let ordered_nodes = graph.topo_for_layout(&provider.repository);
                let layout: GraphLayout = GraphLayout::new(&ordered_nodes);
                Some(GitState {
                    graph,
                    repo: provider.repository.clone(),
                    layout,
                })
            }
        };

        let network = Network::new(None);

        Self {
            screen: Screen::Main,
            git_state,
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
            Screen::Counter(counter) => counter_screen(counter.value),
            Screen::Wordly(wordly_game) => wordly_game.view().map(Message::Wordly),
            Screen::Brain(brain) => brain.view().map(Message::Brain),
            Screen::Main => main_screen(),
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
                    if let Some(git_state) = &self.git_state {
                    scrollable(git_widget(
                        &git_state.layout,
                        &self.git_edge_cache,
                        &self.git_node_cache,
                    ))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    }
                    else{ scrollable(text("Ошибка при парсинге гит графа"))}
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
        background: Some(Background::Color(ACCENT.scale_alpha(0.45))),
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
        background: Some(Background::Color(SURFACE_1)),
        border: Border {
            radius: 0.0.into(),
            width: 1.0,
            color: DIVIDER,
        },
        ..Default::default()
    }
}

fn menu_link_button(label: &str, message: Message) -> button::Button<'_, Message> {
    button(text(label).size(15).center())
        .on_press(message)
        .padding(Padding::from([12, 18]))
        .width(Length::Fill)
        .style(design::ghost_button)
}

fn main_screen() -> Element<'static, Message> {
    let panel = container(
        column![
            container(text("").width(Length::Fixed(56.0)).height(Length::Fixed(3.0)))
                .style(design::accent_strip),
            text("MULTITUL").font(DISPLAY_FONT).size(54),
            text("Сборник iced-экспериментов в одном окне")
                .size(14)
                .style(|_| iced::widget::text::Style { color: Some(TEXT_DIM) }),
            container(text("")).height(12),
            menu_link_button(
                "01 · Counter",
                Message::SwitchTo(Screen::Counter(Counter::default())),
            ),
            menu_link_button(
                "02 · Wordly",
                Message::SwitchTo(Screen::Wordly(Wordly::default())),
            ),
            menu_link_button(
                "03 · One Brain",
                Message::SwitchTo(Screen::Brain(Brain::default())),
            ),
        ]
        .spacing(12)
        .align_x(Horizontal::Center),
    )
    .padding(Padding::from([28, 32]))
    .width(Length::Fixed(360.0))
    .style(design::surface);

    container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .padding(24)
        .into()
}

fn counter_screen(value: i32) -> Element<'static, Message> {
    let panel = container(
        column![
            container(text("").width(Length::Fixed(56.0)).height(Length::Fixed(3.0)))
                .style(design::accent_strip),
            text("COUNTER").font(DISPLAY_FONT).size(36),
            text("Простейший пример состояния")
                .size(13)
                .style(|_| iced::widget::text::Style { color: Some(TEXT_DIM) }),
            container(text("")).height(8),
            text(format!("{}", value))
                .font(DISPLAY_FONT)
                .size(72)
                .style(|_| iced::widget::text::Style { color: Some(ACCENT) }),
            container(text("")).height(8),
            row![
                button(text("−").size(20).center())
                    .on_press(Message::Counter(CounterMessage::Decrement))
                    .padding(Padding::from([10, 22]))
                    .style(design::ghost_button),
                button(text("+").size(20).center())
                    .on_press(Message::Counter(CounterMessage::Increment))
                    .padding(Padding::from([10, 22]))
                    .style(design::primary_button),
            ]
            .spacing(10),
            container(text("")).height(4),
            button(text("В меню").size(14).center())
                .on_press(Message::SwitchTo(Screen::Main))
                .padding(Padding::from([10, 18]))
                .width(Length::Fill)
                .style(design::ghost_button),
        ]
        .spacing(10)
        .align_x(Horizontal::Center),
    )
    .padding(Padding::from([28, 32]))
    .width(Length::Fixed(320.0))
    .style(design::surface);

    container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .padding(24)
        .into()
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
