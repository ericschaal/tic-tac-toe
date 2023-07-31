use std::io;
use anyhow::Result;
use std::io::Write;
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;

pub fn render(stdout: &mut io::Stdout, previous_frame: &Frame, next_frame: &Frame, force: bool) -> Result<()> {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue))?;
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(SetBackgroundColor(Color::Black))?;
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