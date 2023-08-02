use std::thread;
use std::time::Duration;

pub struct FrameRate {}

impl FrameRate {
    pub fn cap(fps: u64, delta: &Duration) {
        let target_delta = 1000.0 / fps as f64;
        let offset = target_delta.floor() - delta.as_millis() as f64;

        if offset > 0.0 {
            thread::sleep(Duration::from_millis(offset.floor() as u64));
        }
    }
}