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

    pub fn to_frame_coordinates(&self) -> FrameCoordinates {
        FrameCoordinates {
            x: 1 + self.x * 4,
            y: self.y * 2
        }
    }
}