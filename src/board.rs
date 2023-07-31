use crate::draw::Drawable;
use crate::frame::{Frame, FrameCoordinates};
use crate::{NUM_COLS, NUM_ROWS};

#[derive(Clone, PartialEq)]
pub enum CellState {
    ONE, TWO, EMPTY
}

pub enum MoveDirection {
    UP, DOWN, LEFT, RIGHT
}

type BoardState = Vec<Vec<CellState>>;

pub struct Board {
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
            let col = vec![CellState::EMPTY; 3];
            state.push(col)
        }

        Self {
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

    fn find_next_available_cell(&self, direction: MoveDirection) -> Option<BoardCoordinates> {

        match direction {
            MoveDirection::UP => {
                let col = self.get_col(self.cursor_position.x);
                col.iter()
                    .enumerate()
                    .rev()
                    .find(|(y, cell)| *y < self.cursor_position.y && **cell == CellState::EMPTY)
                    .map(|(y, _)| BoardCoordinates::new(self.cursor_position.x, y))
            }
            MoveDirection::DOWN => {
                let col = self.get_col(self.cursor_position.x);
                col.iter()
                    .enumerate()
                    .find(|(y, cell)| *y > self.cursor_position.y && **cell == CellState::EMPTY)
                    .map(|(x, _)| BoardCoordinates::new(self.cursor_position.x, x))
            }
            MoveDirection::RIGHT => {
                let row = self.get_row(self.cursor_position.y);
                row.iter()
                    .enumerate()
                    .find(|(x, cell )| *x > self.cursor_position.x && **cell == CellState::EMPTY)
                    .map(|(x, _)| BoardCoordinates::new(x, self.cursor_position.y))

            }
            MoveDirection::LEFT => {
                let row = self.get_row(self.cursor_position.y);
                row.iter()
                    .enumerate()
                    .rev()
                    .find(|(x, cell )| *x < self.cursor_position.x && **cell == CellState::EMPTY)
                    .map(|(x, _)| BoardCoordinates::new(x, self.cursor_position.y))

            }
            _ => None
        }
    }

    fn get_col(&self, x: usize) -> Vec<CellState> {
        self.state.iter().map(|row| row[x].clone()).collect()
    }
    fn get_row(&self, y: usize) -> Vec<CellState> {
        self.state[y].clone()
    }

    fn move_cursor(&mut self, direction: MoveDirection) {
        if let Some(coords) = self.find_next_available_cell(direction) {
            self.cursor_position = coords
        }
    }
}

impl Drawable for Board {
    fn draw(&self, frame: &mut Frame) {
        for y in 0..NUM_ROWS {
            for x in 0..NUM_COLS {
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

                let pos = self.cursor_position.to_frame_coordinates();
                if self.cursor_visible && x == pos.x && y == pos.y {
                    frame[x][y] = "X"
                }
            }
        }
    }
}
