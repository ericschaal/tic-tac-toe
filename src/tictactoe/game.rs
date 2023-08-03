use crate::tictactoe::coordinates::BoardCoordinates;
use crate::tictactoe::movement::MoveDirection;
use crate::tictactoe::player::Player;

type BoardState = Vec<Vec<Player>>;

const BOARD_SIZE: usize = 3;

pub struct Game {
    pub turn: Player,
    pub winner: Player,
    pub state: BoardState,
    pub cursor_position: BoardCoordinates,
}

impl Default for Game {
    fn default() -> Self {
        let mut state = Vec::with_capacity(3);
        for _ in 0..BOARD_SIZE {
            let col = vec![Player::NONE; BOARD_SIZE];
            state.push(col)
        }

        Self {
            turn: Player::ONE,
            winner: Player::NONE,
            state,
            cursor_position: BoardCoordinates::new(0, 0),
        }
    }
}

impl Game {
    pub fn move_cursor(&mut self, direction: MoveDirection) {
        if let Some(new_pos) = self.find_next_available_cell(&self.cursor_position, &direction) {
            self.cursor_position = new_pos;
            return;
        }

        // If no empty cell found in the desired direction, move diagonally
        match direction {
            MoveDirection::UP | MoveDirection::DOWN => {
                for x in 0..BOARD_SIZE {
                    let new_pos = BoardCoordinates::new(x, self.cursor_position.y);
                    if let Some(new_pos) = self.find_next_available_cell(&new_pos, &direction) {
                        self.cursor_position = new_pos;
                        return;
                    }
                }
            }
            MoveDirection::LEFT | MoveDirection::RIGHT => {
                for y in 0..BOARD_SIZE {
                    let new_pos = BoardCoordinates::new(self.cursor_position.x, y);
                    if let Some(new_pos) = self.find_next_available_cell(&new_pos, &direction) {
                        self.cursor_position = new_pos;
                        return;
                    }
                }
            }
        }
    }

    pub fn play(&mut self) {
        if !self.is_game_over() {
            self.state[self.cursor_position.x][self.cursor_position.y] = self.turn.clone();
            self.turn = match self.turn {
                Player::ONE => Player::TWO,
                Player::TWO => Player::ONE,
                _ => Player::ONE
            };
            if let Some(empty_cell) = self.find_empty_cell() {
                self.cursor_position = empty_cell;
            }
            self.winner = self.compute_winner();
        }
    }

    fn find_empty_cell(&self) -> Option<BoardCoordinates> {
        for (x, col) in self.state.iter().enumerate() {
            for (y, cell) in col.iter().enumerate() {
                if *cell == Player::NONE {
                    return Some(BoardCoordinates { x, y });
                }
            }
        }
        None
    }

    fn find_next_available_cell(&self, pos: &BoardCoordinates, direction: &MoveDirection) -> Option<BoardCoordinates> {
        match direction {
            MoveDirection::UP => {
                let row = self.get_row(pos.x);
                row.iter()
                    .enumerate()
                    .rev()
                    .find(|(y, cell)| *y < pos.y && **cell == Player::NONE)
                    .map(|(y, _)| BoardCoordinates::new(pos.x, y))
            }
            MoveDirection::DOWN => {
                let row = self.get_row(pos.x);
                row.iter()
                    .enumerate()
                    .find(|(y, cell)| *y > pos.y && **cell == Player::NONE)
                    .map(|(x, _)| BoardCoordinates::new(pos.x, x))
            }
            MoveDirection::RIGHT => {
                let col = self.get_col(pos.y);
                col.iter()
                    .enumerate()
                    .find(|(x, cell )| *x > pos.x && **cell == Player::NONE)
                    .map(|(x, _)| BoardCoordinates::new(x, pos.y))

            }
            MoveDirection::LEFT => {
                let col = self.get_col(pos.y);
                col.iter()
                    .enumerate()
                    .rev()
                    .find(|(x, cell )| *x < pos.x && **cell == Player::NONE)
                    .map(|(x, _)| BoardCoordinates::new(x, pos.y))

            }
        }
    }

    pub fn is_grid_filled(&self) -> bool {
        self.state.iter().all(|row| row.iter().all(|cell| *cell != Player::NONE))
    }

    pub fn is_game_over(&self) -> bool {
        self.winner != Player::NONE || self.is_grid_filled()
    }

    fn compute_winner(&self) -> Player {
        for row in &self.state {
            if row.iter().all(|cell| *cell == Player::ONE) {
                return Player::ONE;
            }
            if row.iter().all(|cell| *cell == Player::TWO) {
                return Player::TWO;
            }
        }

        // Check columns
        for y in 0..BOARD_SIZE {
            if self.state.iter().all(|row| row[y] == Player::ONE) {
                return Player::ONE;
            }
            if self.state.iter().all(|row| row[y] == Player::TWO) {
                return Player::TWO;
            }
        }

        // Check main diagonal
        if (0..BOARD_SIZE).all(|i| self.state[i][i] == Player::ONE) {
            return Player::ONE;
        }
        if (0..BOARD_SIZE).all(|i| self.state[i][i] == Player::TWO) {
            return Player::TWO;
        }

        // Check anti-diagonal
        if (0..BOARD_SIZE).all(|i| self.state[i][2 - i] == Player::ONE) {
            return Player::ONE;
        }
        if (0..BOARD_SIZE).all(|i| self.state[i][2 - i] == Player::TWO) {
            return Player::TWO;
        }

        Player::NONE
    }

    fn get_col(&self, y: usize) -> Vec<Player> {
        self.state.iter().map(|row| row[y].clone()).collect()
    }

    fn get_row(&self, x: usize) -> Vec<Player> {
        self.state[x].clone()
    }

}