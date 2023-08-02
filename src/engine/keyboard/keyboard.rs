use std::time::Duration;

use anyhow::Result;
use crossterm::event;
use crossterm::event::{Event, KeyCode};

pub struct Keyboard {}

impl Keyboard {
    pub fn poll_keys() -> Result<Vec<KeyCode>> {
        let mut keys = Vec::new();
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                keys.push(key_event.code);
            }
        }

        Ok(keys)
    }
}