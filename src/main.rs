mod games;
use crate::games::wordly::{Wordly, WordlyMessage};

use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::{button, column, container, stack, text};
use iced::{event, keyboard, Color, Element, Event, Length, Point, Rectangle, Renderer, Size, Subscription, Theme};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(theme)
        .subscription(App::subscription)
        .run()
}

struct App {
    screen: Screen,
    hovered_canvas_item: Option<CanvasItem>,
}

fn sign_widget<'a>() -> Element<'a, Message> {
    column![
        text("Multitul"),
        text("by @rubi_ck")
    ].into()
}

fn sign_overlay<'a>() -> Element<'a, Message> {
    container(sign_widget())
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .into()
}

fn canvas_overlay<'a>(app: &App) -> Element<'a, Message> {
    let hover_text = app
        .hovered_canvas_item
        .map(CanvasItem::description)
        .unwrap_or("Наведи курсор на цветной блок в canvas.");

    container(
        column![
            iced::widget::canvas(DemoCanvas)
                .width(280)
                .height(120),
            text(hover_text),
        ]
        .spacing(8),
    )
    .padding(16)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Bottom)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CanvasItem {
    Counter,
    Wordly,
    Tools,
}

impl CanvasItem {
    const ALL: [CanvasItem; 3] = [
        CanvasItem::Counter,
        CanvasItem::Wordly,
        CanvasItem::Tools,
    ];

    fn bounds(self) -> Rectangle {
        match self {
            CanvasItem::Counter => Rectangle {
                x: 20.0,
                y: 20.0,
                width: 70.0,
                height: 70.0,
            },
            CanvasItem::Wordly => Rectangle {
                x: 105.0,
                y: 20.0,
                width: 70.0,
                height: 70.0,
            },
            CanvasItem::Tools => Rectangle {
                x: 190.0,
                y: 20.0,
                width: 70.0,
                height: 70.0,
            },
        }
    }

    fn color(self) -> Color {
        match self {
            CanvasItem::Counter => Color::from_rgb8(0xC2, 0x5B, 0x56),
            CanvasItem::Wordly => Color::from_rgb8(0x3E, 0x8E, 0x7E),
            CanvasItem::Tools => Color::from_rgb8(0x4E, 0x78, 0xA0),
        }
    }

    fn active_color(self) -> Color {
        match self {
            CanvasItem::Counter => Color::from_rgb8(0xF0, 0x84, 0x7C),
            CanvasItem::Wordly => Color::from_rgb8(0x63, 0xC1, 0xA8),
            CanvasItem::Tools => Color::from_rgb8(0x73, 0xA8, 0xD7),
        }
    }

    fn description(self) -> &'static str {
        match self {
            CanvasItem::Counter => "Counter: здесь можно показывать описание счетчика.",
            CanvasItem::Wordly => "Wordly: здесь может быть подсказка или статус игры.",
            CanvasItem::Tools => "Tools: сюда удобно выводить tooltip, превью или метаданные.",
        }
    }
}

#[derive(Debug, Default)]
struct DemoCanvasState {
    hovered: Option<CanvasItem>,
}

struct DemoCanvas;

impl DemoCanvas {
    fn hovered_item(cursor: mouse::Cursor, bounds: Rectangle) -> Option<CanvasItem> {
        let position = cursor.position_in(bounds)?;

        CanvasItem::ALL
            .into_iter()
            .find(|item| item.bounds().contains(position))
    }
}

impl canvas::Program<Message> for DemoCanvas {
    type State = DemoCanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. })
            | canvas::Event::Mouse(mouse::Event::CursorLeft) => {
                let hovered = Self::hovered_item(cursor, bounds);

                if state.hovered != hovered {
                    state.hovered = hovered;

                    return Some(
                        canvas::Action::publish(Message::CanvasHovered(hovered))
                            .and_capture(),
                    );
                }

                if hovered.is_some() {
                    return Some(canvas::Action::capture());
                }

                None
            }
            _ if Self::hovered_item(cursor, bounds).is_some() => {
                Some(canvas::Action::capture())
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let background = canvas::Path::rectangle(Point::ORIGIN, bounds.size());
        frame.fill(&background, Color::from_rgb8(0x1A, 0x1E, 0x24));
        frame.stroke(
            &background,
            canvas::Stroke::default()
                .with_width(1.0)
                .with_color(Color::from_rgb8(0x4B, 0x52, 0x5C)),
        );

        for item in CanvasItem::ALL {
            let item_bounds = item.bounds();
            let shape = canvas::Path::rectangle(
                Point::new(item_bounds.x, item_bounds.y),
                Size::new(item_bounds.width, item_bounds.height),
            );

            let fill = if state.hovered == Some(item) {
                item.active_color()
            } else {
                item.color()
            };

            frame.fill(&shape, fill);
            frame.stroke(
                &shape,
                canvas::Stroke::default()
                    .with_width(if state.hovered == Some(item) { 3.0 } else { 1.0 })
                    .with_color(Color::WHITE),
            );
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if Self::hovered_item(cursor, bounds).is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl App {
    fn new() -> Self {
        Self{
            screen: Screen::Main,
            hovered_canvas_item: None,
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
            Message::CanvasHovered(item) => {
                app.hovered_canvas_item = item;
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
            sign_overlay(),
            canvas_overlay(app)
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
    KeyPressed(KeyMessage),
    CanvasHovered(Option<CanvasItem>),
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
