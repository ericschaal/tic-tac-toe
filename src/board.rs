use crate::draw::Drawable;
use crate::frame::{Frame, FrameCoordinates};
use crate::{NUM_COLS, NUM_ROWS};

#[derive(Clone, PartialEq, Debug)]
pub enum Player {
    ONE, TWO, EMPTY
}

pub enum MoveDirection {
    UP, DOWN, LEFT, RIGHT
}

type BoardState = Vec<Vec<Player>>;

pub struct Board {
    turn: Player,
    state: BoardState,
    cursor_position: BoardCoordinates,
    cursor_visible: bool,
}

#[derive(Default)]
pub struct BoardCoordinates {
    pub x: usize,
    pub y: usize,
}

impl BoardCoordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
    pub fn to_frame_coordinates(&self) -> FrameCoordinates {
        FrameCoordinates {
            x: 1 + self.x * 4,
            y: self.y * 2
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut state = Vec::with_capacity(3);
        for _ in 0..3 {
            let col = vec![Player::EMPTY; 3];
            state.push(col)
        }

        Self {
            turn: Player::ONE,
            state,
            cursor_position: BoardCoordinates::new(2, 2),
            cursor_visible: true,
        }
    }
}

impl Board {
    pub fn move_cursor_down(&mut self) {
        self.move_cursor(MoveDirection::DOWN);
    }

    pub fn move_cursor_up(&mut self) {
        self.move_cursor(MoveDirection::UP);
    }

    pub fn move_cursor_left(&mut self) {
        self.move_cursor(MoveDirection::LEFT);
    }

    pub fn move_cursor_right(&mut self) {
        self.move_cursor(MoveDirection::RIGHT);
    }

    pub fn play(&mut self) {
        self.state[self.cursor_position.x][self.cursor_position.y] = self.turn.clone();
        self.turn = match self.turn {
            Player::ONE => Player::TWO,
            Player::TWO => Player::ONE,
            _ => Player::ONE
        };
        if let Some(empty_cell) = self.find_empty_cell() {
            self.cursor_position = empty_cell;
        }
    }

    fn find_empty_cell(&self) -> Option<BoardCoordinates> {
        for (x, col) in self.state.iter().enumerate() {
            for (y, cell) in col.iter().enumerate() {
                if *cell == Player::EMPTY {
                    return Some(BoardCoordinates { x, y });
                }
            }
        }
        None
    }

    fn find_next_available_cell(&self, direction: MoveDirection) -> Option<BoardCoordinates> {
        match direction {
            MoveDirection::UP => {
                let row = self.get_row(self.cursor_position.x);
                row.iter()
                    .enumerate()
                    .rev()
                    .find(|(y, cell)| *y < self.cursor_position.y && **cell == Player::EMPTY)
                    .map(|(y, _)| BoardCoordinates::new(self.cursor_position.x, y))
            }
            MoveDirection::DOWN => {
                let row = self.get_row(self.cursor_position.x);
                row.iter()
                    .enumerate()
                    .find(|(y, cell)| *y > self.cursor_position.y && **cell == Player::EMPTY)
                    .map(|(x, _)| BoardCoordinates::new(self.cursor_position.x, x))
            }
            MoveDirection::RIGHT => {
                let col = self.get_col(self.cursor_position.y);
                col.iter()
                    .enumerate()
                    .find(|(x, cell )| *x > self.cursor_position.x && **cell == Player::EMPTY)
                    .map(|(x, _)| BoardCoordinates::new(x, self.cursor_position.y))

            }
            MoveDirection::LEFT => {
                let col = self.get_col(self.cursor_position.y);
                col.iter()
                    .enumerate()
                    .rev()
                    .find(|(x, cell )| *x < self.cursor_position.x && **cell == Player::EMPTY)
                    .map(|(x, _)| BoardCoordinates::new(x, self.cursor_position.y))

            }
            _ => None
        }
    }

    fn get_col(&self, y: usize) -> Vec<Player> {
        self.state.iter().map(|row| row[y].clone()).collect()
    }
    fn get_row(&self, x: usize) -> Vec<Player> {
        self.state[x].clone()
    }

    fn move_cursor(&mut self, direction: MoveDirection) {
        if let Some(coords) = self.find_next_available_cell(direction) {
            self.cursor_position = coords
        }
    }
}

impl Drawable for Board {
    fn draw(&self, frame: &mut Frame) {

        for x in 0..3 {
            for y in 0..3 {
                let marker_coords = BoardCoordinates::new(x, y).to_frame_coordinates();
                frame[marker_coords.x][marker_coords.y] = match self.state[x][y] {
                    Player::ONE => "X",
                    Player::TWO => "O",
                    Player::EMPTY => " ",
                };
            }
        }

        let cursor_coords = self.cursor_position.to_frame_coordinates();
        frame[cursor_coords.x][cursor_coords.y] = match self.turn {
            Player::ONE => "x",
            Player::TWO => "o",
            _ => ""
        };

        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                let is_horizontal_line_row = (y + 1) % 2 == 0;
                let is_vertical_line_col = (x + 1) % 4 == 0;

                // Horizontal lines
                if is_horizontal_line_row {
                    frame[x][y] = "━";
                }

                // Vertical lines
                if is_vertical_line_col {
                    frame[x][y] = "┃";
                }
            }
        }
    }
}
