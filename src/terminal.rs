use std::io;
use std::io::Stdout;

use anyhow::Result;
use crossterm::{cursor, ExecutableCommand, terminal};

pub struct Terminal {
    stdout: Stdout,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            stdout: io::stdout()
        }
    }
}

impl Terminal {
    pub fn setup(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        self.stdout.execute(cursor::Hide)?;

        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<()> {
        self.stdout.execute(cursor::Show)?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}