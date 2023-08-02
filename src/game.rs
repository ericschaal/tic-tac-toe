use std::mem::take;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::KeyCode;

use crate::board::Board;
use crate::delta::Delta;
use crate::draw::Drawable;
use crate::frame::new_frame;
use crate::keyboard::Keyboard;
use crate::renderer::Renderer;
use crate::terminal::Terminal;
use crate::updatable::Updatable;

pub struct TicTacToe {
    board: Board,
    fps_target: f64,
    logic_fns: Vec<fn(&mut TicTacToe, &mut Board)>,
    should_stop: bool,
}

impl Default for TicTacToe {
    fn default() -> Self {
        Self {
            board: Board::default(),
            fps_target: 60.0,
            logic_fns: vec![],
            should_stop: false
        }
    }
}

impl TicTacToe {

    pub fn add_logic(mut self, logic_fn: fn(&mut TicTacToe, &mut Board)) -> Self {
        self.logic_fns.push(logic_fn);
        self
    }

    pub fn set_fps(mut self, target: f64) -> Self {
        self.fps_target = target;
        self
    }

    pub fn run(&mut self) -> Result<()> {

        let mut delta_holder = Delta::default();
        let mut renderer = Renderer::default();
        let mut terminal = Terminal::default();

        let mut logic_fns = take(&mut self.logic_fns);
        let mut board = take(&mut self.board);

        renderer.run();
        terminal.setup()?;

        while !self.should_stop {
            // Delta Computation
            let delta = delta_holder.elapsed();
            delta_holder.reset();

            // Frame
            let mut frame = new_frame();

            // Run logic functions
            logic_fns.iter_mut().for_each(|logic_fn| {
               logic_fn(self, &mut board)
            });

            // Run updates
            board.update(delta);

            // Draw
            board.draw(&mut frame);

            // Send Frame
            renderer.send(frame)?;

            if let Some(sleep_duration) = Duration::from_secs_f64(1.0 / self.fps_target).checked_sub(delta) {
                thread::sleep(sleep_duration);
            }

        }

        renderer.stop();
        terminal.cleanup()?;

        Ok(())

    }

    pub fn get_keys(&self) -> Result<Vec<KeyCode>> {
        Keyboard::poll_keys()
    }

    pub fn stop(&mut self) {
        self.should_stop = true;
    }

}