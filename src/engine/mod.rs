use anyhow::Result;
use std::collections::HashMap;
use std::mem::take;
use std::thread;
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::engine::drawable::Drawable;
use crate::engine::framerate::FrameRate;
use crate::engine::keyboard::keyboard::Keyboard;
use crate::engine::rendering::frame;
use crate::engine::rendering::renderer::Renderer;
use crate::engine::rendering::terminal::Terminal;
use crate::engine::sprite::Sprite;
use crate::engine::timing::delta::Delta;
use crate::engine::window::Window;

pub mod sprite;
pub mod window;

mod framerate;
mod keyboard;
mod drawable;
mod rendering;
mod timing;

pub struct Engine<State> {
    fps: u64,
    window: Window,
    sprites: HashMap<String, Sprite>,
    pub delta: Duration,
    pressed_keys: Vec<KeyCode>,
    logic_fns: Vec<fn(&mut Engine<State>, &mut State)>,
    stop: bool,
}

impl<State> Default for Engine<State> {
    fn default() -> Self {
        Self {
            fps: 60,
            window: Window::new(100, 100),
            sprites: HashMap::default(),
            logic_fns: vec![],
            delta: Duration::from_millis(0),
            pressed_keys: vec![],
            stop: false
        }
    }
}

impl<State> Engine<State> {
    
    pub fn new(window: Window) -> Self {
        Self {
            window,
            fps: 60,
            sprites: HashMap::default(),
            logic_fns: vec![],
            delta: Duration::from_millis(0),
            pressed_keys: vec![],
            stop: false
        }
    }
    
    pub fn set_fps(mut self, fps: u64) -> Self {
        self.fps = fps;
        self
    }

    pub fn with_sprite(mut self, label: &str, sprite: Sprite) -> Self {
        self.sprites.insert(label.to_string(), sprite);
        self
    }

    pub fn insert_sprite(&mut self, label: &str, sprite: Sprite) {
        self.sprites.insert(label.into(), sprite);
    }

    pub fn remove_sprite(&mut self, label: &str) {
        self.sprites.remove(label);
    }

    pub fn get_sprite(&self, label: &str) -> Option<&Sprite> {
        self.sprites.get(label)
    }

    pub fn with_logic(mut self, logic_fn: fn(&mut Engine<State>, &mut State)) -> Self {
        self.logic_fns.push(logic_fn);
        self
    }

    pub fn run(&mut self, mut state: State) -> Result<()> {
        self.stop = false;

        let mut delta_holder = Delta::default();
        let mut renderer = Renderer::new(self.window.width, self.window.height);
        let mut terminal = Terminal::default();

        let mut logic_fns = take(&mut self.logic_fns);

        renderer.run();
        terminal.setup()?;

        while !self.stop {
            // Delta Computation
            self.delta = delta_holder.elapsed();
            delta_holder.reset();

            // Frame
            let mut frame = frame::new_frame(self.window.width, self.window.height);

            // Get Keyboard keys
            self.pressed_keys = Keyboard::poll_keys()?;

            // Logic
            for logic_fn in logic_fns.iter_mut() {
                logic_fn(self, &mut state);
            }

            // Draw
            let mut sprites: Vec<&Sprite> = self.sprites.values().collect();
            sprites.sort_by(|a,b| a.layer.partial_cmp(&b.layer).unwrap());
            for sprite in sprites.iter() {
                sprite.draw(&mut frame);
            }

            // Send Frame
            renderer.send(frame)?;

            FrameRate::cap(self.fps, &self.delta);

        }

        renderer.stop();
        terminal.cleanup()?;

        Ok(())

    }

    pub fn get_pressed_keys(&self) -> Vec<KeyCode> {
        self.pressed_keys.clone()
    }

    pub fn stop(&mut self) {
        self.stop = true;
    }
}

