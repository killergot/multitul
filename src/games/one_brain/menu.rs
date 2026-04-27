use iced::widget::{button, column, text, text_input};
use iced::{Element, Subscription, Task};

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
}

impl Default for Brain {
    fn default() -> Self {
        Self {
            state: BrainState::Menu,
            current_room_id: None,
            current_player_name: None,
            chat: Vec::new(),
            server_url: String::from("ws://127.0.0.1:8765"),
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
                let room_id = self.room_id_input.trim();
                if room_id.is_empty() {
                    self.error = Some(String::from("Введите идентификатор комнаты."));
                    return Task::none();
                }

                let name = self.name_input.trim();
                if name.is_empty() {
                    self.error = Some(String::from("Введите имя игрока."));
                    return Task::none();
                }

                self.current_room_id = Some(room_id.to_string());
                self.current_player_name = Some(name.to_string());
                self.players = vec![name.to_string()];
                self.ready_count = 0;
                self.total_players = 2;
                self.round = 1;
                self.finished = false;
                self.chat.clear();
                self.chat_input.clear();
                self.word_input.clear();
                self.error = None;
                self.push_system_message(format!(
                    "Локальная комната {} открыта. Следующим шагом сюда подключим WebSocket.",
                    room_id
                ));
                self.state = BrainState::InRoom;

                Task::none()
            }
            BrainMessage::SendWord => {
                let word = self.word_input.trim();
                if word.is_empty() {
                    self.error = Some(String::from("Введите слово перед отправкой."));
                    return Task::none();
                }

                self.ready_count = 1;
                self.error = None;
                self.push_system_message(format!("Слово \"{}\" подготовлено локально.", word));
                self.word_input.clear();

                Task::none()
            }
            BrainMessage::SendChat => {
                let message = self.chat_input.trim();
                if message.is_empty() {
                    self.error = Some(String::from("Введите сообщение для чата."));
                    return Task::none();
                }

                let sender_name = self
                    .current_player_name
                    .clone()
                    .unwrap_or_else(|| String::from("Player"));
                self.chat.push(ChatLine {
                    sender_name,
                    text: message.to_string(),
                    timestamp: 0.0,
                });
                self.chat_input.clear();
                self.error = None;

                Task::none()
            }
            BrainMessage::LeaveRoom => {
                self.reset_room_state();
                self.state = BrainState::SelectRoom;
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<BrainMessage> {
        Subscription::none()
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

    fn reset_room_state(&mut self) {
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
        self.error = None;
    }
}

#[derive(Default, Clone, Debug)]
pub enum BrainState {
    #[default]
    Menu,
    SelectRoom,
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
}
