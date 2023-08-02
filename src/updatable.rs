use std::time::Duration;

pub trait Updatable {
    fn update(&mut self, delta: Duration);
}