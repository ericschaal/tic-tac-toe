use vector2d::Vector2D;

#[derive(Default, Clone, Copy)]
pub struct BoardCoordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Clone, Copy)]
pub struct FrameCoordinates {
    pub x: usize,
    pub y: usize
}

impl BoardCoordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn to_frame_coordinates(&self, board_offset: &Vector2D<usize>) -> FrameCoordinates {
        FrameCoordinates {
            x: 1 + self.x * 4 + board_offset.x,
            y: self.y * 2 + board_offset.y
        }
    }
}