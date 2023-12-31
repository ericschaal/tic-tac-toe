use crate::engine::rendering::frame::Frame;

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}