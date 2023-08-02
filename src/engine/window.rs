pub struct Window {
    pub height: usize,
    pub width: usize,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height
        }
    }
}