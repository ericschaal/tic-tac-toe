use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

#[derive(Default)]
pub struct FrameCoordinates {
    pub x: usize,
    pub y: usize
}

pub fn new_frame() -> Frame {
    let mut frame = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let col = vec![" "; NUM_ROWS];
        frame.push(col);
    };

    frame
}
