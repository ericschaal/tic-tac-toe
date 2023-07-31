use std::{io, thread};
use std::time::{Duration, Instant};
use anyhow::Result;
use crossterm::{
    terminal,
    cursor,
    ExecutableCommand,
    event,
    event::{Event}
};
use crossterm::event::KeyCode;
use tic_tac_toe::board::Board;
use tic_tac_toe::draw::Drawable;
use tic_tac_toe::{frame, render};

const FRAME_PER_S: f64 = 60.0;

fn main() -> Result<()> {

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let (render_tx, render_rx) = crossbeam_channel::unbounded();

    let render_handle = thread::spawn(move || {
        let mut previous_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render::render(&mut stdout, &previous_frame, &previous_frame, true).unwrap();

        while let Ok(next_frame) = render_rx.recv() {
            render::render(&mut stdout, &previous_frame, &next_frame, false).unwrap();
            previous_frame = next_frame;
        }
    });

    let mut instant = Instant::now();
    let mut board = Board::default();

    'gameloop: loop {

        let delta = instant.elapsed();
        instant = Instant::now();
        let mut next_frame = frame::new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Left => {
                        board.move_cursor_left();
                    }
                    KeyCode::Right => {
                        board.move_cursor_right();
                    }
                    KeyCode::Up => {
                        board.move_cursor_up();
                    }
                    KeyCode::Down => {
                        board.move_cursor_down();
                    }
                    _ => {}
                }
            }
        }

        let drawables: Vec<&dyn Drawable> = vec![&board];
        drawables.iter()
            .for_each(|d| d.draw(&mut next_frame));

        render_tx.send(next_frame)?;

        if let Some(sleep_duration) = Duration::from_secs_f64(1.0 / FRAME_PER_S).checked_sub(delta) {
            thread::sleep(sleep_duration);
        }

    };

    drop(render_tx);
    render_handle.join().unwrap();

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())

}
