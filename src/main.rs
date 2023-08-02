use anyhow::Result;
use crossterm::event::KeyCode;

use tic_tac_toe::board::Board;
use tic_tac_toe::game::TicTacToe;
use tic_tac_toe::movement::MoveDirection;

const FRAME_PER_S: f64 = 60.0;


fn main() -> Result<()> {
    TicTacToe::default()
        .add_logic(movement_logic)
        .set_fps(FRAME_PER_S)
        .run()?;

    Ok(())
}

fn movement_logic(engine: &mut TicTacToe, board: &mut Board) {
    for key in engine.get_keys().unwrap().iter() {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                engine.stop();
            }
            KeyCode::Left => {
                board.move_cursor(MoveDirection::LEFT);
            }
            KeyCode::Right => {
                board.move_cursor(MoveDirection::RIGHT);
            }
            KeyCode::Up => {
                board.move_cursor(MoveDirection::UP);
            }
            KeyCode::Down => {
                board.move_cursor(MoveDirection::DOWN);
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                board.play();
            }
            _ => {}
        }
    }
}
