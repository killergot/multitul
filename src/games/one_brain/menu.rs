use futures::SinkExt;
use iced::futures::channel::mpsc;
use iced::widget::{
    Id, button, column, container, operation, row, scrollable, scrollable::Scrollbar, text,
    text_input,
};
use iced::{Element, Length, Padding, Subscription, Task};

use crate::games::one_brain::protocol::{ChatItem, ClientMessage, ServerMessage};
use crate::games::one_brain::styles;
use crate::games::one_brain::ws::{self, WsCommand, WsConfig};

static CHAT_SCROLLABLE_ID: Id = Id::new("one_brain_chat");
static ROUND_SCROLLABLE_ID: Id = Id::new("one_brain_rounds");

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
    last_submitted_word: Option<String>,
    round_summaries: Vec<RoundSummary>,
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
            last_submitted_word: None,
            round_summaries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChatLine {
    pub sender_name: String,
    pub text: String,
    pub timestamp: f64,
}

#[derive(Debug, Clone)]
struct RoundSummary {
    round: u32,
    is_match: bool,
    words: Vec<(String, String)>,
}

impl Brain {
    pub fn view(&self) -> Element<'_, BrainMessage> {
        let stage = match self.state {
            BrainState::Menu => self.menu_stage_view(),
            BrainState::SelectRoom => self.join_form_stage_view(),
            BrainState::Connecting => self.connecting_stage_view(),
            BrainState::InRoom => self.room_stage_view(false),
            BrainState::FinishedGame => self.room_stage_view(true),
        };

        let layout = row![
            container(
                row![
                    container(self.sidebar_view())
                        .width(240)
                        .height(Length::Fill)
                        .padding(20)
                        .style(styles::sidebar_panel),
                    container(stage)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(22)
                        .style(styles::stage_panel),
                ]
                .spacing(16)
                .height(Length::Fill)
            )
            .width(Length::FillPortion(2))
            .height(Length::Fill),
            container(self.chat_panel_view())
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .padding(20)
                .style(styles::chat_panel),
        ]
        .spacing(16)
        .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .style(styles::screen_shell)
            .into()
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
            BrainMessage::WsEvent(event) => self.apply_server_message(event),
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
                let word = self.word_input.trim().to_string();
                if word.is_empty() {
                    self.error = Some(String::from("Введите слово перед отправкой."));
                    return Task::none();
                }

                self.error = None;
                self.last_submitted_word = Some(word.clone());
                let task =
                    self.send_ws_command(WsCommand::Send(ClientMessage::SubmitWord { word }));
                self.word_input.clear();

                task
            }
            BrainMessage::SendChat => {
                let message = self.chat_input.trim().to_string();
                if message.is_empty() {
                    self.error = Some(String::from("Введите сообщение для чата."));
                    return Task::none();
                }

                self.error = None;
                let task = self.send_ws_command(WsCommand::Send(ClientMessage::ChatMessage {
                    text: message,
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

    fn sidebar_view(&self) -> Element<'_, BrainMessage> {
        let mut primary_button = button(text("Открыть комнату").size(15))
            .width(Length::Fill)
            .padding(Padding::from([12, 16]))
            .style(styles::primary_button);

        if !matches!(self.state, BrainState::Connecting) {
            primary_button = primary_button.on_press(BrainMessage::OpenJoinForm);
        }

        let mut leave_button = button(text("Покинуть комнату").size(15))
            .width(Length::Fill)
            .padding(Padding::from([12, 16]))
            .style(styles::danger_button);

        if self.is_live_state() {
            leave_button = leave_button.on_press(BrainMessage::LeaveRoom);
        }

        let status_card = container(
            column![
                text("Состояние").size(13),
                text(self.state_title()).size(24),
                text(self.state_subtitle()).size(14),
            ]
            .spacing(8),
        )
        .padding(Padding::from([16, 18]))
        .width(Length::Fill)
        .style(styles::accent_card);

        let mut column = column![
            text("ONE BRAIN").size(30),
            text("Два игрока, одна мысль, один экран контроля.").size(14),
            status_card,
            primary_button,
            leave_button,
            button(text("Назад").size(15))
                .width(Length::Fill)
                .padding(Padding::from([12, 16]))
                .style(styles::secondary_button)
                .on_press(BrainMessage::GoHome),
            button(text("Список комнат").size(15))
                .width(Length::Fill)
                .padding(Padding::from([12, 16]))
                .style(styles::secondary_button),
        ]
        .spacing(12);

        if let Some(error) = &self.error {
            column = column.push(
                container(column![text("Ошибка").size(13), text(error).size(14)].spacing(6))
                    .padding(Padding::from([12, 14]))
                    .width(Length::Fill)
                    .style(styles::system_bubble),
            );
        }

        column.into()
    }

    fn menu_stage_view(&self) -> Element<'_, BrainMessage> {
        column![
            self.stage_header_view(
                "Синхронизация мыслей",
                "Слева управление комнатой, справа чат, а центральная зона отведена под сам раунд и историю ваших слов."
            )
        ]
        .spacing(18)
        .height(Length::Fill)
        .into()
    }

    fn join_form_stage_view(&self) -> Element<'_, BrainMessage> {
        let connect_button = button(text("Подключиться").size(13).center())
            .padding(Padding::from([10, 14]))
            .width(Length::Fill)
            .style(styles::primary_button)
            .on_press(BrainMessage::ConnectPressed);

        column![
            self.stage_header_view(
                "Подключение к комнате",
                "Соберите room id и имя — и сцена переключится в боевой режим."
            ),
            container(
                column![
                    text("Форма входа").size(16),
                    text_input("room id", &self.room_id_input)
                        .on_input(BrainMessage::RoomIdChanged)
                        .style(styles::warm_input)
                        .padding(Padding::from([10, 12]))
                        .size(15),
                    text_input("имя игрока", &self.name_input)
                        .on_input(BrainMessage::NameChanged)
                        .style(styles::game_input)
                        .padding(Padding::from([10, 12]))
                        .size(15),
                    connect_button,
                ]
                .spacing(10),
            )
            .padding(Padding::from([14, 16]))
            .width(Length::Fill)
            .style(styles::card),
            row![
                self.feature_card(
                    "Подсказка",
                    "Название комнаты должно совпасть у обоих игроков."
                ),
                self.feature_card("Лайв-чат", "Чат активируется в правой колонке."),
            ]
            .spacing(10),
        ]
        .spacing(12)
        .into()
    }

    fn connecting_stage_view(&self) -> Element<'_, BrainMessage> {
        column![
            self.stage_header_view("Соединение на линии", "Канал открыт, ждём подтверждение."),
            row![
                self.stat_card("Сервер", &self.server_url),
                self.stat_card("Комната", &self.room_id_input),
                self.stat_card("Игрок", &self.name_input),
            ]
            .spacing(10),
            container(
                column![
                    text("Ожидание joined / room_state").size(15),
                    text("Как только сервер пришлёт joined — переключим сцену на раунд и журнал.")
                        .size(13)
                        .style(|_| iced::widget::text::Style {
                            color: Some(crate::utils::style::TEXT_DIM)
                        }),
                ]
                .spacing(6),
            )
            .padding(Padding::from([14, 16]))
            .width(Length::Fill)
            .style(styles::accent_card),
        ]
        .spacing(12)
        .into()
    }

    fn room_stage_view(&self, is_finished: bool) -> Element<'_, BrainMessage> {
        let status = if is_finished || self.finished {
            "Финал"
        } else {
            "Активный раунд"
        };

        let mut send_word_button = button(text("Отправить слово").size(13).center())
            .padding(Padding::from([10, 14]))
            .style(styles::primary_button);

        if !is_finished {
            send_word_button = send_word_button.on_press(BrainMessage::SendWord);
        }

        let player_list = if self.players.is_empty() {
            column![text("Игроки появятся после room_state.").size(13)]
        } else {
            self.players.iter().fold(column![], |column, player| {
                column.push(text(format!("• {}", player)).size(13))
            })
        };

        column![
            self.stage_header_view(
                "Игровая сцена",
                "Текущий ввод, последняя отправка и история совпадений."
            ),
            row![
                self.stat_card(
                    "Комната",
                    self.current_room_id
                        .as_deref()
                        .unwrap_or(self.room_id_input.as_str()),
                ),
                self.stat_card(
                    "Игрок",
                    self.current_player_name
                        .as_deref()
                        .unwrap_or(self.name_input.as_str()),
                ),
                self.stat_card("Раунд", &self.round.to_string()),
                self.stat_card("Статус", status),
            ]
            .spacing(8),
            row![
                container(
                    column![
                        text("Пульт слова").size(15),
                        text_input("Введите слово", &self.word_input)
                            .on_input(BrainMessage::WordInputChanged)
                            .on_submit(BrainMessage::SendWord)
                            .style(styles::game_input)
                            .padding(Padding::from([10, 12]))
                            .size(15),
                        row![
                            self.word_info_card(
                                "Последняя отправка",
                                self.last_submitted_word
                                    .as_deref()
                                    .unwrap_or("Ещё не отправлялось"),
                            ),
                        ]
                        .spacing(8),
                        send_word_button,
                    ]
                    .spacing(8),
                )
                .padding(Padding::from([12, 14]))
                .width(Length::FillPortion(2))
                .style(styles::accent_card),
                container(
                    column![
                        text("Ритм комнаты").size(15),
                        text(format!("Готовность: {}/{}", self.ready_count, self.total_players))
                            .size(13),
                        text(if self.finished {
                            "Оба игрока уже сошлись."
                        } else if self.ready_count > 0 {
                            "Кто-то уже отправил слово."
                        } else {
                            "Раунд ещё не начинался."
                        })
                        .size(13),
                        text("Игроки").size(13),
                        player_list,
                    ]
                    .spacing(6),
                )
                .padding(Padding::from([12, 14]))
                .width(Length::FillPortion(1))
                .style(styles::card),
            ]
            .spacing(8),
            container(
                column![
                    text("Журнал слов").size(15),
                    self.round_history_view(),
                ]
                .spacing(8),
            )
            .padding(Padding::from([12, 14]))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(styles::card),
        ]
        .spacing(10)
        .height(Length::Fill)
        .into()
    }

    fn chat_panel_view(&self) -> Element<'_, BrainMessage> {
        let messages = if self.chat.is_empty() {
            column![
                container(
                    column![
                        text("Чат спит").size(18),
                        text("После входа сюда будут приходить системные сообщения и реплики игроков.")
                            .size(14),
                    ]
                    .spacing(6),
                )
                .padding(Padding::from([14, 16]))
                .width(Length::Fill)
                .style(styles::system_bubble)
            ]
        } else {
            self.chat.iter().fold(column![], |column, line| {
                column.push(self.chat_bubble_view(line))
            })
        };

        let scroll = scrollable(container(messages.spacing(10)).padding(Padding {
            right: 6.0,
            ..Padding::ZERO
        }))
        .id(CHAT_SCROLLABLE_ID.clone())
        .direction(scrollable::Direction::Vertical(
            Scrollbar::new().width(8).margin(2).scroller_width(8),
        ))
        .style(styles::panel_scrollable)
        .height(Length::Fill);

        let mut send_button = button(text("Отправить").size(15))
            .padding(Padding::from([12, 16]))
            .width(Length::Fill)
            .style(styles::secondary_button);

        if matches!(self.state, BrainState::InRoom | BrainState::FinishedGame) {
            send_button = send_button.on_press(BrainMessage::SendChat);
        }

        column![
            text("Чат комнаты").size(26),
            text("Правая колонка остаётся отдельной, чтобы разговор не забивал игровую сцену.")
                .size(14),
            scroll,
            text_input("Сообщение в чат", &self.chat_input)
                .on_input(BrainMessage::ChatInputChanged)
                .on_submit(BrainMessage::SendChat)
                .style(styles::warm_input)
                .padding(Padding::from([12, 14]))
                .size(16),
            send_button,
        ]
        .spacing(12)
        .height(Length::Fill)
        .into()
    }

    fn stage_header_view<'a>(
        &self,
        title: &'a str,
        subtitle: &'a str,
    ) -> Element<'a, BrainMessage> {
        container(
            column![
                text(title).size(20),
                text(subtitle)
                    .size(12)
                    .style(|_| iced::widget::text::Style {
                        color: Some(crate::utils::style::TEXT_DIM)
                    }),
            ]
            .spacing(4),
        )
        .padding(Padding::from([12, 14]))
        .width(Length::Fill)
        .style(styles::accent_card)
        .into()
    }

    fn feature_card<'a>(&self, title: &'a str, body: &'a str) -> Element<'a, BrainMessage> {
        container(
            column![
                text(title).size(13),
                text(body).size(12).style(|_| iced::widget::text::Style {
                    color: Some(crate::utils::style::TEXT_DIM)
                }),
            ]
            .spacing(4),
        )
        .padding(Padding::from([10, 12]))
        .width(Length::FillPortion(1))
        .style(styles::card)
        .into()
    }

    fn stat_card(
        &self,
        title: impl Into<String>,
        value: impl Into<String>,
    ) -> Element<'_, BrainMessage> {
        let title = title.into();
        let value = value.into();

        container(
            column![
                text(title).size(11).style(|_| iced::widget::text::Style {
                    color: Some(crate::utils::style::TEXT_DIM)
                }),
                text(value).size(14),
            ]
            .spacing(3),
        )
        .padding(Padding::from([8, 10]))
        .width(Length::FillPortion(1))
        .style(styles::card)
        .into()
    }

    fn word_info_card(
        &self,
        title: impl Into<String>,
        value: impl Into<String>,
    ) -> Element<'_, BrainMessage> {
        let title = title.into();
        let value = value.into();

        container(
            column![
                text(title).size(11).style(|_| iced::widget::text::Style {
                    color: Some(crate::utils::style::TEXT_DIM)
                }),
                text(value).size(13),
            ]
            .spacing(3),
        )
        .padding(Padding::from([8, 10]))
        .width(Length::FillPortion(1))
        .style(styles::word_card)
        .into()
    }

    fn round_history_view(&self) -> Element<'_, BrainMessage> {
        let content = if self.round_summaries.is_empty() {
            column![
                container(
                    column![
                        text("Пока раунды не завершались.").size(13),
                        text("После round_result здесь появятся карточки с обоими словами.")
                            .size(12)
                            .style(|_| iced::widget::text::Style {
                                color: Some(crate::utils::style::TEXT_DIM)
                            }),
                    ]
                    .spacing(4),
                )
                .padding(Padding::from([10, 12]))
                .width(Length::Fill)
                .style(styles::word_card)
            ]
        } else {
            self.round_summaries
                .iter()
                .fold(column![], |column, summary| {
                    let words = if summary.words.is_empty() {
                        column![text("Слова ещё не зафиксированы.").size(12)]
                    } else {
                        summary
                            .words
                            .iter()
                            .fold(column![], |column, (player, word)| {
                                column.push(text(format!("{} → {}", player, word)).size(12))
                            })
                    };

                    column.push(
                        container(
                            column![
                                text(format!("Раунд {}", summary.round)).size(14),
                                text(if summary.is_match {
                                    "Совпадение найдено"
                                } else {
                                    "Мысли ещё расходятся"
                                })
                                .size(12)
                                .style(|_| {
                                    iced::widget::text::Style {
                                        color: Some(crate::utils::style::TEXT_DIM),
                                    }
                                }),
                                words,
                            ]
                            .spacing(4),
                        )
                        .padding(Padding::from([10, 12]))
                        .width(Length::Fill)
                        .style(move |theme| styles::round_card(theme, summary.is_match)),
                    )
                })
        };

        scrollable(container(content.spacing(10)).padding(Padding {
            right: 6.0,
            ..Padding::ZERO
        }))
        .id(ROUND_SCROLLABLE_ID.clone())
        .direction(scrollable::Direction::Vertical(
            Scrollbar::new().width(8).margin(2).scroller_width(8),
        ))
        .style(styles::panel_scrollable)
        .height(Length::Fill)
        .into()
    }

    fn chat_bubble_view<'a>(&self, line: &'a ChatLine) -> Element<'a, BrainMessage> {
        let is_system = line.sender_name.eq_ignore_ascii_case("system");

        container(
            column![
                text(line.sender_name.as_str()).size(14),
                text(line.text.as_str()).size(15),
            ]
            .spacing(6),
        )
        .padding(Padding::from([12, 14]))
        .width(Length::Fill)
        .style(if is_system {
            styles::system_bubble
        } else {
            styles::player_bubble
        })
        .into()
    }

    fn state_title(&self) -> &'static str {
        match self.state {
            BrainState::Menu => "Холл",
            BrainState::SelectRoom => "Форма входа",
            BrainState::Connecting => "Подключение",
            BrainState::InRoom => "В комнате",
            BrainState::FinishedGame => "Финал",
        }
    }

    fn state_subtitle(&self) -> &'static str {
        match self.state {
            BrainState::Menu => "Экран ожидания перед подключением.",
            BrainState::SelectRoom => "Подготовьте room id и имя.",
            BrainState::Connecting => "Канал открыт, ждём подтверждение.",
            BrainState::InRoom => "Раунд в процессе, можно общаться и отправлять слово.",
            BrainState::FinishedGame => "Партия завершена, история сохранена ниже.",
        }
    }

    fn is_live_state(&self) -> bool {
        matches!(
            self.state,
            BrainState::Connecting | BrainState::InRoom | BrainState::FinishedGame
        )
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
        self.last_submitted_word = None;
        self.round_summaries.clear();
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

    fn apply_server_message(&mut self, message: ServerMessage) -> Task<BrainMessage> {
        let mut chat_changed = false;
        let mut rounds_changed = false;

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
                chat_changed = true;
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
                chat_changed = true;
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
                let mut words: Vec<(String, String)> = words.into_iter().collect();
                words.sort_by(|left, right| left.0.cmp(&right.0));

                self.round_summaries.push(RoundSummary {
                    round,
                    is_match,
                    words: words.clone(),
                });
                self.last_submitted_word = None;

                let result = if is_match {
                    format!("Раунд {} завершён совпадением.", round)
                } else {
                    format!("Раунд {} завершён без совпадения.", round)
                };
                self.chat.push(ChatLine {
                    sender_name: String::from("System"),
                    text: result,
                    timestamp: 0.0,
                });
                chat_changed = true;
                rounds_changed = true;
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
                self.chat.push(ChatLine {
                    sender_name: String::from("System"),
                    text: format!("Игра завершена. Общее слово: {}", word),
                    timestamp: 0.0,
                });
                chat_changed = true;
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

        let mut tasks = Vec::new();
        if chat_changed {
            tasks.push(operation::snap_to_end(CHAT_SCROLLABLE_ID.clone()));
        }
        if rounds_changed {
            tasks.push(operation::snap_to_end(ROUND_SCROLLABLE_ID.clone()));
        }
        Task::batch(tasks)
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
