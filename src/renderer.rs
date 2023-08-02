use std::{io, thread};
use std::io::Write;
use std::thread::JoinHandle;

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType};

use crate::frame;
use crate::frame::Frame;

pub struct Renderer {
    handle: Option<JoinHandle<()>>,
    rx: Receiver<Frame>,
    tx: Sender<Frame>
}

impl Default for Renderer {
    fn default() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Self { tx, rx, handle: None }
    }
}

impl Renderer {

    pub fn run(&mut self) {
        let rx = std::mem::replace(&mut self.rx, crossbeam_channel::never());

        let render_handle = thread::spawn(move || {
            let mut previous_frame = frame::new_frame();
            let mut stdout = io::stdout();

            Renderer::render(&mut stdout, &previous_frame, &previous_frame, true).unwrap();

            while let Ok(next_frame) = rx.recv() {
                Renderer::render(&mut stdout, &previous_frame, &next_frame, false).unwrap();
                previous_frame = next_frame;
            }
        });

        self.handle = Some(render_handle);
    }

    pub fn send(&self, frame: Frame) -> Result<()> {
        self.tx.send(frame)?;
        Ok(())
    }

    pub fn stop(mut self) {
        if let Some(handle) = self.handle {
            drop(self.rx);
            drop(self.tx);

            handle.join().unwrap();
            self.handle = None;

        }
    }

    fn render(stdout: &mut io::Stdout, previous_frame: &Frame, next_frame: &Frame, force: bool) -> Result<()> {
        if force {
            stdout.queue(Clear(ClearType::All))?;
        }
        for (x, col) in next_frame.iter().enumerate() {
            for (y, content) in col.iter().enumerate() {
                let content_changed = previous_frame[x][y] != *content;
                if content_changed || force {
                    stdout.queue(MoveTo(x as u16, y as u16))?;
                    print!("{}", *content);
                }

            }
        }

        stdout.flush()?;

        Ok(())
    }
}