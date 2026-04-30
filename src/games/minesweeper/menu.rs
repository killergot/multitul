use std::time::Duration;

use super::model::{BoardConfig, Cell, CellState, DifficultyPreset, GameSession, GameStatus};
use super::styles;
use crate::utils::style::{self, ACCENT, DISPLAY_FONT, TEXT_DIM};
use iced::alignment::{Horizontal, Vertical};
use iced::mouse;
use iced::time::every;
use iced::widget::{button, column, container, mouse_area, row, scrollable, text};
use iced::{Element, Length, Padding, Subscription};

const CELL_SIZE: f32 = 30.0;

#[derive(Debug, Clone)]
pub struct Minesweeper {
    state: MinesweeperState,
    custom: CustomConfig,
    game: GameSession,
}

impl Default for Minesweeper {
    fn default() -> Self {
        let config = BoardConfig::beginner();

        Self {
            state: MinesweeperState::Menu,
            custom: CustomConfig::default(),
            game: GameSession::new(config),
        }
    }
}

impl Minesweeper {
    pub fn view(&self) -> Element<'_, MinesweeperMessage> {
        match self.state {
            MinesweeperState::Menu => self.menu_view(),
            MinesweeperState::Playing => self.game_view(),
        }
    }

    pub fn update(&mut self, message: MinesweeperMessage) {
        match message {
            MinesweeperMessage::GoHome => {}
            MinesweeperMessage::StartPreset(preset) => {
                let config = match preset {
                    DifficultyPreset::Beginner => BoardConfig::beginner(),
                    DifficultyPreset::Intermediate => BoardConfig::intermediate(),
                    DifficultyPreset::Expert => BoardConfig::expert(),
                    DifficultyPreset::Custom => self.custom.as_board_config(),
                };

                self.start_game(config);
            }
            MinesweeperMessage::CustomWidthStep(delta) => {
                self.custom.adjust_width(delta);
            }
            MinesweeperMessage::CustomHeightStep(delta) => {
                self.custom.adjust_height(delta);
            }
            MinesweeperMessage::CustomMinesStep(delta) => {
                self.custom.adjust_mines(delta);
            }
            MinesweeperMessage::StartCustom => {
                self.start_game(self.custom.as_board_config());
            }
            MinesweeperMessage::RevealCell(index) => {
                self.game.reveal(index);
            }
            MinesweeperMessage::ToggleFlag(index) => {
                self.game.toggle_flag(index);
            }
            MinesweeperMessage::Restart => {
                self.start_game(self.game.config());
            }
            MinesweeperMessage::BackToSetup => {
                self.state = MinesweeperState::Menu;
            }
            MinesweeperMessage::Tick => {
                self.game.tick();
            }
        }
    }

    pub fn subscription(&self) -> Subscription<MinesweeperMessage> {
        if self.state == MinesweeperState::Playing && self.game.status() == GameStatus::Playing {
            every(Duration::from_secs(1)).map(|_| MinesweeperMessage::Tick)
        } else {
            Subscription::none()
        }
    }

    fn start_game(&mut self, config: BoardConfig) {
        self.game = GameSession::new(config);
        self.state = MinesweeperState::Playing;
    }

    fn menu_view(&self) -> Element<'_, MinesweeperMessage> {
        let panel = container(
            column![
                container(
                    text("")
                        .width(Length::Fixed(56.0))
                        .height(Length::Fixed(3.0))
                )
                .style(style::accent_strip),
                text("MINESWEEPER").font(DISPLAY_FONT).size(44),
                text("Choose a preset or build a custom board.")
                    .size(14)
                    .style(|_| iced::widget::text::Style {
                        color: Some(TEXT_DIM)
                    }),
                self.preset_button(
                    DifficultyPreset::Beginner,
                    "9x9, 10 mines. Fast board for testing and warmup.",
                ),
                self.preset_button(
                    DifficultyPreset::Intermediate,
                    "16x16, 40 mines. Balanced default for regular play.",
                ),
                self.preset_button(
                    DifficultyPreset::Expert,
                    "30x16, 99 mines. Wide board with high density.",
                ),
                self.custom_panel(),
                menu_button("Back to main menu", MinesweeperMessage::GoHome),
            ]
            .spacing(12)
            .align_x(Horizontal::Center),
        )
        .padding(Padding::from([28, 32]))
        .width(Length::Fixed(520.0))
        .style(styles::shell);

        container(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(24)
            .into()
    }

    fn game_view(&self) -> Element<'_, MinesweeperMessage> {
        let config = self.game.config();
        let status_text = match self.game.status() {
            GameStatus::Ready => "Ready",
            GameStatus::Playing => "Playing",
            GameStatus::Won => "Won",
            GameStatus::Lost => "Lost",
        };

        let summary = if self.game.status() == GameStatus::Won {
            "All safe cells are open."
        } else if self.game.status() == GameStatus::Lost {
            "Mine triggered. Right click toggles flags on the next run."
        } else {
            "Left click opens a cell. Right click toggles a flag."
        };

        let layout = row![
            container(
                column![
                    text("MINESWEEPER").font(DISPLAY_FONT).size(32),
                    text(summary).size(13).style(|_| iced::widget::text::Style {
                        color: Some(TEXT_DIM)
                    }),
                    container(
                        column![
                            self.stat_row("Board", format!("{} x {}", config.width, config.height)),
                            self.stat_row("Mines", config.mines.to_string()),
                            self.stat_row("Flags left", self.game.flags_left().to_string()),
                            self.stat_row("Timer", format!("{}s", self.game.elapsed_seconds())),
                            self.stat_row("State", status_text),
                        ]
                        .spacing(8),
                    )
                    .padding(Padding::from([14, 16]))
                    .style(styles::accent_card),
                    menu_button("Restart", MinesweeperMessage::Restart),
                    menu_button("Choose difficulty", MinesweeperMessage::BackToSetup),
                    menu_button("Back to main menu", MinesweeperMessage::GoHome),
                ]
                .spacing(12)
            )
            .width(Length::Fixed(260.0))
            .height(Length::Fill),
            container(self.board_view())
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(16)
                .style(styles::board_frame),
        ]
        .spacing(16)
        .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .into()
    }

    fn preset_button(
        &self,
        preset: DifficultyPreset,
        subtitle: &'static str,
    ) -> Element<'_, MinesweeperMessage> {
        let config = match preset {
            DifficultyPreset::Beginner => BoardConfig::beginner(),
            DifficultyPreset::Intermediate => BoardConfig::intermediate(),
            DifficultyPreset::Expert => BoardConfig::expert(),
            DifficultyPreset::Custom => self.custom.as_board_config(),
        };

        let label = format!(
            "{}  {}x{}  {} mines",
            preset.label(),
            config.width,
            config.height,
            config.mines
        );

        container(
            column![
                text(label).size(15),
                text(subtitle).size(12).style(|_| iced::widget::text::Style {
                    color: Some(TEXT_DIM)
                }),
                button(text("Start").size(14).center())
                    .on_press(MinesweeperMessage::StartPreset(preset))
                    .padding(Padding::from([10, 16]))
                    .style(styles::primary_button),
            ]
            .spacing(8),
        )
        .padding(Padding::from([14, 16]))
        .width(Length::Fill)
        .style(styles::card)
        .into()
    }

    fn custom_panel(&self) -> Element<'_, MinesweeperMessage> {
        let config = self.custom.as_board_config();
        let density = ((config.mines as f32 / config.cell_count() as f32) * 100.0).round() as i32;

        container(
            column![
                text("Custom board").size(16),
                text("Clamp-based controls keep the board valid, so start is always available.")
                    .size(12)
                    .style(|_| iced::widget::text::Style {
                        color: Some(TEXT_DIM)
                    }),
                self.stepper_row(
                    "Width",
                    config.width,
                    MinesweeperMessage::CustomWidthStep(-1),
                    MinesweeperMessage::CustomWidthStep(1),
                ),
                self.stepper_row(
                    "Height",
                    config.height,
                    MinesweeperMessage::CustomHeightStep(-1),
                    MinesweeperMessage::CustomHeightStep(1),
                ),
                self.stepper_row(
                    "Mines",
                    config.mines,
                    MinesweeperMessage::CustomMinesStep(-1),
                    MinesweeperMessage::CustomMinesStep(1),
                ),
                text(format!(
                    "Cells: {}  Density: {}%",
                    config.cell_count(),
                    density
                ))
                .size(12)
                .style(|_| iced::widget::text::Style {
                    color: Some(ACCENT)
                }),
                button(text("Start custom board").size(14).center())
                    .on_press(MinesweeperMessage::StartCustom)
                    .padding(Padding::from([10, 16]))
                    .style(styles::primary_button),
            ]
            .spacing(10),
        )
        .padding(Padding::from([14, 16]))
        .width(Length::Fill)
        .style(styles::accent_card)
        .into()
    }

    fn stepper_row(
        &self,
        label: &'static str,
        value: usize,
        decrement: MinesweeperMessage,
        increment: MinesweeperMessage,
    ) -> Element<'_, MinesweeperMessage> {
        row![
            text(label).size(13).width(Length::FillPortion(2)),
            button(text("-").size(14).center())
                .on_press(decrement)
                .padding(Padding::from([6, 12]))
                .style(styles::ghost_button),
            container(text(value.to_string()).font(DISPLAY_FONT).size(18))
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            button(text("+").size(14).center())
                .on_press(increment)
                .padding(Padding::from([6, 12]))
                .style(styles::ghost_button),
        ]
        .spacing(10)
        .align_y(Vertical::Center)
        .into()
    }

    fn stat_row(
        &self,
        label: impl Into<String>,
        value: impl Into<String>,
    ) -> Element<'static, MinesweeperMessage> {
        let label = label.into();
        let value = value.into();

        row![
            text(label).size(12).width(Length::FillPortion(1)).style(
                |_| iced::widget::text::Style {
                    color: Some(TEXT_DIM)
                }
            ),
            text(value).size(13).width(Length::FillPortion(1)),
        ]
        .spacing(8)
        .into()
    }

    fn board_view(&self) -> Element<'_, MinesweeperMessage> {
        let config = self.game.config();
        let exploded_index = self.game.exploded_index();
        let status = self.game.status();

        let rows = (0..config.height).map(|row_index| {
            row((0..config.width).map(|column_index| {
                let index = row_index * config.width + column_index;
                let cell = self.game.cells()[index];
                self.cell_view(index, cell, status, exploded_index == Some(index))
            }))
            .spacing(2)
            .into()
        });

        scrollable(
            container(column(rows).spacing(2))
                .padding(8)
                .width(Length::Shrink)
                .height(Length::Shrink),
        )
        .style(styles::scrollable)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn cell_view(
        &self,
        index: usize,
        cell: Cell,
        status: GameStatus,
        is_exploded: bool,
    ) -> Element<'_, MinesweeperMessage> {
        let label = match cell.state {
            CellState::Hidden => "",
            CellState::Flagged => "F",
            CellState::Revealed => {
                if cell.has_mine {
                    "*"
                } else {
                    ""
                }
            }
        };

        let content = if cell.state == CellState::Revealed && !cell.has_mine && cell.adjacent_mines > 0
        {
            text(cell.adjacent_mines.to_string())
                .font(DISPLAY_FONT)
                .size(18)
                .style(move |_| iced::widget::text::Style {
                    color: Some(styles::number_color(cell)),
                })
        } else {
            text(label)
                .font(DISPLAY_FONT)
                .size(18)
                .style(move |_| iced::widget::text::Style {
                    color: Some(styles::number_color(cell)),
                })
        };

        mouse_area(
            container(content)
                .width(Length::Fixed(CELL_SIZE))
                .height(Length::Fixed(CELL_SIZE))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(move |theme| styles::cell(theme, cell, status, is_exploded)),
        )
        .on_press(MinesweeperMessage::RevealCell(index))
        .on_right_press(MinesweeperMessage::ToggleFlag(index))
        .interaction(mouse::Interaction::Pointer)
        .into()
    }
}

fn menu_button(
    label: &str,
    message: MinesweeperMessage,
) -> button::Button<'_, MinesweeperMessage> {
    button(text(label).size(15).center())
        .on_press(message)
        .padding(Padding::from([12, 18]))
        .width(Length::Fill)
        .style(styles::ghost_button)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MinesweeperState {
    Menu,
    Playing,
}

#[derive(Debug, Clone)]
pub enum MinesweeperMessage {
    GoHome,
    StartPreset(DifficultyPreset),
    CustomWidthStep(isize),
    CustomHeightStep(isize),
    CustomMinesStep(isize),
    StartCustom,
    RevealCell(usize),
    ToggleFlag(usize),
    Restart,
    BackToSetup,
    Tick,
}

#[derive(Debug, Clone)]
struct CustomConfig {
    width: usize,
    height: usize,
    mines: usize,
}

impl Default for CustomConfig {
    fn default() -> Self {
        Self {
            width: 12,
            height: 12,
            mines: 20,
        }
    }
}

impl CustomConfig {
    fn as_board_config(&self) -> BoardConfig {
        BoardConfig::custom(self.width, self.height, self.mines)
            .validate()
            .unwrap_or_else(|_| BoardConfig::beginner())
    }

    fn adjust_width(&mut self, delta: isize) {
        self.width = step_value(
            self.width,
            delta,
            BoardConfig::MIN_WIDTH,
            BoardConfig::MAX_WIDTH,
        );
        self.mines = self.mines.min(self.max_mines());
    }

    fn adjust_height(&mut self, delta: isize) {
        self.height = step_value(
            self.height,
            delta,
            BoardConfig::MIN_HEIGHT,
            BoardConfig::MAX_HEIGHT,
        );
        self.mines = self.mines.min(self.max_mines());
    }

    fn adjust_mines(&mut self, delta: isize) {
        self.mines = step_value(self.mines, delta, BoardConfig::MIN_MINES, self.max_mines());
    }

    fn max_mines(&self) -> usize {
        self.width * self.height - 1
    }
}

fn step_value(current: usize, delta: isize, min: usize, max: usize) -> usize {
    let stepped = if delta.is_negative() {
        current.saturating_sub(delta.unsigned_abs())
    } else {
        current.saturating_add(delta as usize)
    };

    stepped.clamp(min, max)
}
