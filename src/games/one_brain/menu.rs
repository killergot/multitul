use futures::SinkExt;
use iced::futures::channel::mpsc;
use iced::widget::{button, column, text, text_input};
use iced::{Element, Subscription, Task};

use crate::games::one_brain::protocol::{ChatItem, ClientMessage, ServerMessage};
use crate::games::one_brain::ws::{self, WsCommand, WsConfig};

#[derive(Debug, Clone)]
pub struct Brain {
    state: BrainState,
    current_room_id: Option<String>,
    current_player_name: Option<String>,
    chat: Vec<ChatLine>,
    server_url: String,
    room_id_input: String,
    name_input: String,
    chat_input: String,
    word_input: String,
    players: Vec<String>,
    ready_count: u32,
    total_players: u32,
    round: u32,
    finished: bool,
    error: Option<String>,
    connection_request: Option<WsConfig>,
    ws_sender: Option<mpsc::Sender<WsCommand>>,
}

impl Default for Brain {
    fn default() -> Self {
        Self {
            state: BrainState::Menu,
            current_room_id: None,
            current_player_name: None,
            chat: Vec::new(),
            server_url: String::from("ws://185.200.176.8:8765"),
            room_id_input: String::new(),
            name_input: String::new(),
            chat_input: String::new(),
            word_input: String::new(),
            players: Vec::new(),
            ready_count: 0,
            total_players: 2,
            round: 1,
            finished: false,
            error: None,
            connection_request: None,
            ws_sender: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChatLine {
    pub sender_name: String,
    pub text: String,
    pub timestamp: f64,
}

impl Brain {
    pub fn view(&self) -> Element<'_, BrainMessage> {
        match &self.state {
            BrainState::Menu => self.menu_view(),
            BrainState::SelectRoom => self.join_form_view(),
            BrainState::Connecting => self.connecting_view(),
            BrainState::InRoom => self.room_view(false),
            BrainState::FinishedGame => self.room_view(true),
        }
    }

    pub fn update(&mut self, msg: BrainMessage) -> Task<BrainMessage> {
        match msg {
            BrainMessage::GoHome => Task::none(),
            BrainMessage::OpenJoinForm => {
                self.state = BrainState::SelectRoom;
                self.error = None;
                Task::none()
            }
            BrainMessage::RoomIdChanged(value) => {
                self.room_id_input = value;
                self.error = None;
                Task::none()
            }
            BrainMessage::NameChanged(value) => {
                self.name_input = value;
                self.error = None;
                Task::none()
            }
            BrainMessage::ChatInputChanged(value) => {
                self.chat_input = value;
                self.error = None;
                Task::none()
            }
            BrainMessage::WordInputChanged(value) => {
                self.word_input = value;
                self.error = None;
                Task::none()
            }
            BrainMessage::ConnectPressed => {
                let room_id = self.room_id_input.trim().to_string();
                if room_id.is_empty() {
                    self.error = Some(String::from("Введите идентификатор комнаты."));
                    return Task::none();
                }

                let name = self.name_input.trim().to_string();
                if name.is_empty() {
                    self.error = Some(String::from("Введите имя игрока."));
                    return Task::none();
                }

                self.clear_live_room_state();
                self.error = None;
                self.connection_request = Some(WsConfig {
                    server_url: self.server_url.clone(),
                    room_id,
                    name,
                });
                self.state = BrainState::Connecting;

                Task::none()
            }
            BrainMessage::WsReady(sender) => {
                self.ws_sender = Some(sender);
                Task::none()
            }
            BrainMessage::WsConnected => Task::none(),
            BrainMessage::WsEvent(event) => {
                self.apply_server_message(event);
                Task::none()
            }
            BrainMessage::WsClosed => {
                self.ws_sender = None;
                if self.connection_request.is_some() {
                    self.connection_request = None;

                    if matches!(
                        self.state,
                        BrainState::Connecting | BrainState::InRoom | BrainState::FinishedGame
                    ) {
                        self.clear_live_room_state();
                        self.state = BrainState::SelectRoom;

                        if self.error.is_none() {
                            self.error = Some(String::from("Соединение закрыто."));
                        }
                    }
                }
                Task::none()
            }
            BrainMessage::WsError(message) => {
                self.error = Some(message);
                Task::none()
            }
            BrainMessage::SendWord => {
                let word = self.word_input.trim();
                if word.is_empty() {
                    self.error = Some(String::from("Введите слово перед отправкой."));
                    return Task::none();
                }

                self.error = None;
                let task = self.send_ws_command(WsCommand::Send(ClientMessage::SubmitWord {
                    word: word.to_string(),
                }));
                self.word_input.clear();

                task
            }
            BrainMessage::SendChat => {
                let message = self.chat_input.trim();
                if message.is_empty() {
                    self.error = Some(String::from("Введите сообщение для чата."));
                    return Task::none();
                }

                self.error = None;
                let task = self.send_ws_command(WsCommand::Send(ClientMessage::ChatMessage {
                    text: message.to_string(),
                }));
                self.chat_input.clear();

                task
            }
            BrainMessage::LeaveRoom => {
                self.connection_request = None;
                self.ws_sender = None;
                self.clear_live_room_state();
                self.state = BrainState::SelectRoom;
                Task::none()
            }
            BrainMessage::Noop => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<BrainMessage> {
        match &self.connection_request {
            Some(config) => ws::subscription(config.clone()),
            None => Subscription::none(),
        }
    }

    fn menu_view(&self) -> Element<'_, BrainMessage> {
        column![
            button("Зайти в игру").on_press(BrainMessage::OpenJoinForm),
            button("Посмотреть список комнат"),
            button("Назад").on_press(BrainMessage::GoHome),
        ]
        .spacing(10)
        .into()
    }

    fn join_form_view(&self) -> Element<'_, BrainMessage> {
        let mut content = column![
            text("Подключение к комнате"),
            text(format!("Сервер: {}", self.server_url)),
            text_input("Введите room id", &self.room_id_input)
                .on_input(BrainMessage::RoomIdChanged),
            text_input("Введите имя игрока", &self.name_input)
                .on_input(BrainMessage::NameChanged),
            button("Войти").on_press(BrainMessage::ConnectPressed),
            button("Назад").on_press(BrainMessage::GoHome),
        ]
        .spacing(10);

        if let Some(error) = &self.error {
            content = content.push(text(format!("Ошибка: {}", error)));
        }

        content.into()
    }

    fn connecting_view(&self) -> Element<'_, BrainMessage> {
        let mut content = column![
            text("Подключение к серверу"),
            text(format!("Сервер: {}", self.server_url)),
            text(format!("Комната: {}", self.room_id_input)),
            text(format!("Игрок: {}", self.name_input)),
            button("Отмена").on_press(BrainMessage::LeaveRoom),
        ]
        .spacing(10);

        if let Some(error) = &self.error {
            content = content.push(text(format!("Ошибка: {}", error)));
        }

        content.into()
    }

    fn room_view(&self, is_finished: bool) -> Element<'_, BrainMessage> {
        let room_id = self
            .current_room_id
            .as_deref()
            .unwrap_or("room is not selected");
        let player_name = self
            .current_player_name
            .as_deref()
            .unwrap_or("player is not selected");
        let players_text = if self.players.is_empty() {
            String::from("Пока в комнате никого нет.")
        } else {
            self.players.join(", ")
        };

        let chat_column = if self.chat.is_empty() {
            column![text("Чат пока пуст.")]
        } else {
            self.chat.iter().fold(column![], |column, line| {
                column.push(text(format!("{}: {}", line.sender_name, line.text)))
            })
        };

        let mut content = column![
            text(format!("Комната: {}", room_id)),
            text(format!("Игрок: {}", player_name)),
            text(format!("Игроки: {}", players_text)),
            text(format!(
                "Готовность: {}/{}",
                self.ready_count, self.total_players
            )),
            text(format!("Раунд: {}", self.round)),
            text(if is_finished || self.finished {
                "Игра завершена"
            } else {
                "Игра активна"
            }),
            text_input("Введите слово", &self.word_input).on_input(BrainMessage::WordInputChanged),
            button("Отправить слово").on_press(BrainMessage::SendWord),
            text_input("Сообщение в чат", &self.chat_input)
                .on_input(BrainMessage::ChatInputChanged),
            button("Отправить сообщение").on_press(BrainMessage::SendChat),
            text("Чат"),
            chat_column,
            button("Покинуть комнату").on_press(BrainMessage::LeaveRoom),
        ]
        .spacing(10);

        if let Some(error) = &self.error {
            content = content.push(text(format!("Ошибка: {}", error)));
        }

        content.into()
    }

    fn push_system_message(&mut self, text: String) {
        self.chat.push(ChatLine {
            sender_name: String::from("System"),
            text,
            timestamp: 0.0,
        });
    }

    fn clear_live_room_state(&mut self) {
        self.current_room_id = None;
        self.current_player_name = None;
        self.chat.clear();
        self.chat_input.clear();
        self.word_input.clear();
        self.players.clear();
        self.ready_count = 0;
        self.total_players = 2;
        self.round = 1;
        self.finished = false;
    }

    fn send_ws_command(&mut self, command: WsCommand) -> Task<BrainMessage> {
        let Some(sender) = self.ws_sender.clone() else {
            self.error = Some(String::from("Соединение с сервером ещё не готово."));
            return Task::none();
        };

        Task::perform(
            async move {
                let mut sender = sender;
                let _ = sender.send(command).await;
            },
            |_| BrainMessage::Noop,
        )
    }

    fn apply_server_message(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::Joined {
                player_id: _,
                room_id,
                name,
            } => {
                self.current_room_id = Some(room_id);
                self.current_player_name = Some(name);
                self.state = BrainState::InRoom;
                self.error = None;
            }
            ServerMessage::ChatHistory {
                room_id: _,
                messages,
            } => {
                self.chat = messages.into_iter().map(Self::chat_from_item).collect();
            }
            ServerMessage::ChatMessage {
                room_id: _,
                sender_id: _,
                sender_name,
                text,
                timestamp,
            } => {
                self.chat.push(ChatLine {
                    sender_name,
                    text,
                    timestamp,
                });
            }
            ServerMessage::RoomState {
                room_id: _,
                players,
                ready_count,
                total_players,
                round,
                finished,
            } => {
                self.players = players;
                self.ready_count = ready_count;
                self.total_players = total_players;
                self.round = round;
                self.finished = finished;

                if finished {
                    self.state = BrainState::FinishedGame;
                }
            }
            ServerMessage::RoundResult {
                room_id: _,
                round,
                words,
                is_match,
            } => {
                let result = if is_match {
                    format!("Раунд {} завершён успешно: {:?}", round, words)
                } else {
                    format!("Раунд {} завершён без совпадения: {:?}", round, words)
                };
                self.push_system_message(result);
            }
            ServerMessage::GameOver {
                room_id: _,
                round,
                word,
                history: _,
            } => {
                self.round = round;
                self.finished = true;
                self.state = BrainState::FinishedGame;
                self.push_system_message(format!("Игра завершена. Общее слово: {}", word));
            }
            ServerMessage::Left => {
                self.connection_request = None;
                self.ws_sender = None;
                self.clear_live_room_state();
                self.state = BrainState::SelectRoom;
            }
            ServerMessage::Pong => {}
            ServerMessage::Error { message } => {
                self.error = Some(message);
            }
        }
    }

    fn chat_from_item(item: ChatItem) -> ChatLine {
        ChatLine {
            sender_name: item.sender_name,
            text: item.text,
            timestamp: item.timestamp,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum BrainState {
    #[default]
    Menu,
    SelectRoom,
    Connecting,
    InRoom,
    FinishedGame,
}

#[derive(Clone, Debug)]
pub enum BrainMessage {
    GoHome,
    OpenJoinForm,
    ConnectPressed,
    RoomIdChanged(String),
    NameChanged(String),
    ChatInputChanged(String),
    WordInputChanged(String),
    SendWord,
    SendChat,
    LeaveRoom,
    WsReady(mpsc::Sender<WsCommand>),
    WsConnected,
    WsEvent(ServerMessage),
    WsClosed,
    WsError(String),
    Noop,
}
