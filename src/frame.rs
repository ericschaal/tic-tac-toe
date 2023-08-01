use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<String>>;

#[derive(Default)]
pub struct FrameCoordinates {
    pub x: usize,
    pub y: usize
}

pub fn new_frame() -> Frame {
    let mut frame = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let col = vec![" ".to_string(); NUM_ROWS];
        frame.push(col);
    };

    frame
}
