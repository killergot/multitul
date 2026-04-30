use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyPreset {
    Beginner,
    Intermediate,
    Expert,
    Custom,
}

impl DifficultyPreset {
    pub fn label(self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Expert => "Expert",
            Self::Custom => "Custom",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardConfig {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
    pub preset: DifficultyPreset,
}

impl BoardConfig {
    pub const MIN_WIDTH: usize = 6;
    pub const MAX_WIDTH: usize = 30;
    pub const MIN_HEIGHT: usize = 6;
    pub const MAX_HEIGHT: usize = 24;
    pub const MIN_MINES: usize = 1;

    pub fn beginner() -> Self {
        Self {
            width: 9,
            height: 9,
            mines: 10,
            preset: DifficultyPreset::Beginner,
        }
    }

    pub fn intermediate() -> Self {
        Self {
            width: 16,
            height: 16,
            mines: 40,
            preset: DifficultyPreset::Intermediate,
        }
    }

    pub fn expert() -> Self {
        Self {
            width: 30,
            height: 16,
            mines: 99,
            preset: DifficultyPreset::Expert,
        }
    }

    pub fn custom(width: usize, height: usize, mines: usize) -> Self {
        Self {
            width,
            height,
            mines,
            preset: DifficultyPreset::Custom,
        }
    }

    pub fn validate(self) -> Result<Self, &'static str> {
        if !(Self::MIN_WIDTH..=Self::MAX_WIDTH).contains(&self.width) {
            return Err("width out of bounds");
        }
        if !(Self::MIN_HEIGHT..=Self::MAX_HEIGHT).contains(&self.height) {
            return Err("height out of bounds");
        }
        if self.mines < Self::MIN_MINES {
            return Err("mines out of bounds");
        }
        if self.mines >= self.cell_count() {
            return Err("too many mines");
        }

        Ok(self)
    }

    pub fn cell_count(self) -> usize {
        self.width * self.height
    }

    pub fn max_mines(self) -> usize {
        self.cell_count().saturating_sub(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellState {
    #[default]
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cell {
    pub has_mine: bool,
    pub adjacent_mines: u8,
    pub state: CellState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Ready,
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone)]
pub struct GameSession {
    config: BoardConfig,
    cells: Vec<Cell>,
    status: GameStatus,
    flags_left: isize,
    revealed_safe_cells: usize,
    first_move_done: bool,
    elapsed_seconds: u32,
    exploded_index: Option<usize>,
}

impl GameSession {
    pub fn new(config: BoardConfig) -> Self {
        let config = config.validate().unwrap_or_else(|_| BoardConfig::beginner());

        Self {
            config,
            cells: vec![Cell::default(); config.cell_count()],
            status: GameStatus::Ready,
            flags_left: config.mines as isize,
            revealed_safe_cells: 0,
            first_move_done: false,
            elapsed_seconds: 0,
            exploded_index: None,
        }
    }

    pub fn config(&self) -> BoardConfig {
        self.config
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn flags_left(&self) -> isize {
        self.flags_left
    }

    pub fn elapsed_seconds(&self) -> u32 {
        self.elapsed_seconds
    }

    pub fn exploded_index(&self) -> Option<usize> {
        self.exploded_index
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.status, GameStatus::Won | GameStatus::Lost)
    }

    pub fn reveal(&mut self, index: usize) {
        if self.is_finished() || index >= self.cells.len() {
            return;
        }

        match self.cells[index].state {
            CellState::Flagged => return,
            CellState::Revealed => {
                self.chord(index);
                return;
            }
            CellState::Hidden => {}
        }

        if !self.first_move_done {
            self.plant_mines(index);
            self.first_move_done = true;
            self.status = GameStatus::Playing;
        }

        if self.cells[index].has_mine {
            self.cells[index].state = CellState::Revealed;
            self.exploded_index = Some(index);
            self.reveal_all_mines();
            self.status = GameStatus::Lost;
            return;
        }

        self.reveal_region(index);
        self.refresh_status();
    }

    pub fn toggle_flag(&mut self, index: usize) {
        if self.is_finished() || index >= self.cells.len() {
            return;
        }

        match self.cells[index].state {
            CellState::Revealed => {}
            CellState::Hidden => {
                if self.flags_left == 0 {
                    return;
                }

                self.cells[index].state = CellState::Flagged;
                self.flags_left -= 1;
            }
            CellState::Flagged => {
                self.cells[index].state = CellState::Hidden;
                self.flags_left += 1;
            }
        }
    }

    pub fn tick(&mut self) {
        if self.status == GameStatus::Playing && self.first_move_done {
            self.elapsed_seconds += 1;
        }
    }

    fn chord(&mut self, index: usize) {
        if self.is_finished() || !self.first_move_done || index >= self.cells.len() {
            return;
        }

        let cell = self.cells[index];
        if cell.state != CellState::Revealed || cell.adjacent_mines == 0 {
            return;
        }

        let neighbors = self.neighbor_indices(index);
        let flagged = neighbors
            .iter()
            .filter(|&&neighbor| self.cells[neighbor].state == CellState::Flagged)
            .count();

        if flagged != cell.adjacent_mines as usize {
            return;
        }

        let hidden_neighbors: Vec<usize> = neighbors
            .into_iter()
            .filter(|neighbor| self.cells[*neighbor].state == CellState::Hidden)
            .collect();

        for neighbor in &hidden_neighbors {
            if self.cells[*neighbor].has_mine {
                self.cells[*neighbor].state = CellState::Revealed;
                self.exploded_index = Some(*neighbor);
                self.reveal_all_mines();
                self.status = GameStatus::Lost;
                return;
            }
        }

        for neighbor in hidden_neighbors {
            self.reveal_region(neighbor);
        }

        self.refresh_status();
    }

    fn plant_mines(&mut self, safe_index: usize) {
        let mut positions: Vec<usize> = (0..self.cells.len())
            .filter(|&index| index != safe_index)
            .collect();
        let mut rng = rand::thread_rng();
        positions.shuffle(&mut rng);

        for index in positions.into_iter().take(self.config.mines) {
            self.cells[index].has_mine = true;
        }

        self.recompute_adjacent_counts();
    }

    fn recompute_adjacent_counts(&mut self) {
        for index in 0..self.cells.len() {
            let adjacent = self
                .neighbor_indices(index)
                .into_iter()
                .filter(|neighbor| self.cells[*neighbor].has_mine)
                .count() as u8;
            self.cells[index].adjacent_mines = adjacent;
        }
    }

    fn reveal_region(&mut self, start: usize) {
        let mut stack = vec![start];

        while let Some(index) = stack.pop() {
            if index >= self.cells.len() {
                continue;
            }

            let cell = self.cells[index];
            if cell.state == CellState::Revealed || cell.state == CellState::Flagged || cell.has_mine
            {
                continue;
            }

            self.cells[index].state = CellState::Revealed;
            self.revealed_safe_cells += 1;

            if self.cells[index].adjacent_mines == 0 {
                stack.extend(
                    self.neighbor_indices(index)
                        .into_iter()
                        .filter(|neighbor| self.cells[*neighbor].state != CellState::Revealed),
                );
            }
        }
    }

    fn refresh_status(&mut self) {
        if self.revealed_safe_cells + self.config.mines == self.config.cell_count() {
            self.status = GameStatus::Won;
            self.flags_left = 0;

            for cell in &mut self.cells {
                if cell.has_mine {
                    cell.state = CellState::Flagged;
                }
            }
        } else if self.first_move_done {
            self.status = GameStatus::Playing;
        }
    }

    fn reveal_all_mines(&mut self) {
        for cell in &mut self.cells {
            if cell.has_mine {
                cell.state = CellState::Revealed;
            }
        }
    }

    fn neighbor_indices(&self, index: usize) -> Vec<usize> {
        let x = index % self.config.width;
        let y = index / self.config.width;
        let mut neighbors = Vec::with_capacity(8);

        let min_y = y.saturating_sub(1);
        let max_y = (y + 1).min(self.config.height - 1);
        let min_x = x.saturating_sub(1);
        let max_x = (x + 1).min(self.config.width - 1);

        for row in min_y..=max_y {
            for column in min_x..=max_x {
                let neighbor = row * self.config.width + column;
                if neighbor != index {
                    neighbors.push(neighbor);
                }
            }
        }

        neighbors
    }

    #[cfg(test)]
    fn with_mines(config: BoardConfig, mines: &[usize]) -> Self {
        assert_eq!(config.mines, mines.len());

        let mut session = Self::new(config);
        session.first_move_done = true;
        session.status = GameStatus::Playing;

        for &index in mines {
            session.cells[index].has_mine = true;
        }

        session.recompute_adjacent_counts();
        session
    }
}

#[cfg(test)]
mod tests {
    use super::{BoardConfig, CellState, DifficultyPreset, GameSession, GameStatus};

    #[test]
    fn first_click_is_always_safe() {
        let config = BoardConfig::beginner();
        let mut session = GameSession::new(config);
        let first_click = 40;

        session.reveal(first_click);

        assert_eq!(session.cells[first_click].state, CellState::Revealed);
        assert!(!session.cells[first_click].has_mine);
        assert_ne!(session.status, GameStatus::Lost);
    }

    #[test]
    fn reveal_zero_area_opens_connected_region() {
        let config = BoardConfig::custom(6, 6, 1).validate().unwrap();
        let mut session = GameSession::with_mines(config, &[35]);

        session.reveal(0);

        assert_eq!(session.revealed_safe_cells, 35);
        assert_eq!(session.status, GameStatus::Won);
        assert_eq!(session.cells[35].state, CellState::Flagged);
    }

    #[test]
    fn toggling_flags_updates_counter() {
        let config = BoardConfig::custom(6, 6, 3).validate().unwrap();
        let mut session = GameSession::new(config);

        session.toggle_flag(0);
        session.toggle_flag(1);
        session.toggle_flag(0);

        assert_eq!(session.flags_left(), 2);
        assert_eq!(session.cells[0].state, CellState::Hidden);
        assert_eq!(session.cells[1].state, CellState::Flagged);
    }

    #[test]
    fn chord_reveals_neighbors_when_flags_match() {
        let config = BoardConfig::custom(6, 6, 1).validate().unwrap();
        let mut session = GameSession::with_mines(config, &[0]);

        session.reveal(7);
        session.toggle_flag(0);
        session.reveal(7);

        assert_eq!(session.status, GameStatus::Won);
        assert!(session
            .cells
            .iter()
            .enumerate()
            .all(|(index, cell)| index == 0 || cell.state == CellState::Revealed));
    }

    #[test]
    fn hitting_a_mine_ends_the_game() {
        let config = BoardConfig::custom(6, 6, 1).validate().unwrap();
        let mut session = GameSession::with_mines(config, &[5]);

        session.reveal(5);

        assert_eq!(session.status, GameStatus::Lost);
        assert_eq!(session.exploded_index(), Some(5));
        assert_eq!(session.cells[5].state, CellState::Revealed);
    }

    #[test]
    fn config_rejects_more_mines_than_cells_allow() {
        let invalid = BoardConfig {
            width: 6,
            height: 6,
            mines: 36,
            preset: DifficultyPreset::Custom,
        };

        assert!(invalid.validate().is_err());
    }
}
