pub type Frame = Vec<Vec<u32>>;

#[derive(Default, Debug, Clone, Copy)]
pub struct FrameCoordinates {
    pub x: usize,
    pub y: usize
}

pub fn new_frame(width: usize, height: usize) -> Frame {
    let mut frame = Vec::with_capacity(width);
    for _ in 0..width {
        let col = vec![' ' as u32; height];
        frame.push(col);
    };
    frame
}
