use anyhow::Result;
use crossterm::event::KeyCode;
use vector2d::Vector2D;
use tic_tac_toe::{
    engine::{Engine, sprite::Sprite}
};
use tic_tac_toe::engine::window::Window;
use tic_tac_toe::tictactoe::coordinates::BoardCoordinates;
use tic_tac_toe::tictactoe::game::Game;
use tic_tac_toe::tictactoe::movement::MoveDirection;
use tic_tac_toe::tictactoe::player::Player;

const GRID_HEIGHT: usize = 5;
const GRID_WIDTH: usize = 11;

fn main() -> Result<()> {
    let mut grid = Sprite::from_file("grid".into(), "sprites/grid")?;
    grid.layer = 0;

    Engine::new(Window::new(50, 50))
        .set_fps(60.0)
        .add_sprite("grid".into(), grid)
        .add_logic(base_logic)
        .add_logic(game_movement)
        .add_logic(cursor_logic)
        .add_logic(markers_logic)
        .add_logic(win_lose_logic)
        .run(Game::default())?;

    Ok(())
}

fn base_logic<T>(engine: &mut Engine<T>, _: &mut T) {
    for key in engine.get_pressed_keys() {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                engine.stop();
            }
            _ => {}
        }
    }
}

fn game_movement(engine: &mut Engine<Game>, game: &mut Game) {
    for key in engine.get_pressed_keys() {
        match key {
            KeyCode::Left => {
                game.move_cursor(MoveDirection::LEFT);
            }
            KeyCode::Right => {
                game.move_cursor(MoveDirection::RIGHT);
            }
            KeyCode::Up => {
                game.move_cursor(MoveDirection::UP);
            }
            KeyCode::Down => {
                game.move_cursor(MoveDirection::DOWN);
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                game.play();
            }
            _ => {}
        }
    }
}

fn cursor_logic(engine: &mut Engine<Game>, game: &mut Game) {
    if !game.is_game_over() {
        let cursor_str = if game.turn == Player::ONE {"x"} else {"o"};
        let mut sprite = Sprite::from_string("cursor".into(), cursor_str.into());
        let cursor_coords = game.cursor_position.to_frame_coordinates();
        let translate = Vector2D::new(cursor_coords.x, cursor_coords.y);

        sprite.translation = translate;
        sprite.layer = 1;

        if game.turn == Player::ONE {
            engine.sprites.remove("p2");
            engine.sprites.insert("p1".into(), sprite);
        } else if game.turn == Player::TWO {
            engine.sprites.remove("p1");
            engine.sprites.insert("p2".into(), sprite);
        }
    }
}

fn markers_logic(engine: &mut Engine<Game>, game: &mut Game) {
    for (x, col) in game.state.iter().enumerate() {
        for (y, cell) in col.iter().enumerate() {
            if *cell == Player::ONE || *cell == Player::TWO {
                let marker_str = if *cell == Player::ONE {"X"} else {"O"};
                let mut sprite = Sprite::from_string("marker".into(), marker_str.into());
                let translate = BoardCoordinates::new(x, y).to_frame_coordinates();

                sprite.translation = Vector2D::new(translate.x, translate.y);
                sprite.layer = 1;

                engine.sprites.insert(format!("{}-{}", x, y), sprite);
            }
        }
    }
}

fn win_lose_logic(engine: &mut Engine<Game>, game: &mut Game) {

    let str = if game.winner != Player::NONE {
        format!("Winner: {:?}", game.winner)
    } else if game.is_grid_filled() {
        "Draw".to_string()
    } else {
        format!("Turn: Player {:?}", game.turn)
    };

    engine.sprites.remove("win-lose");

    let mut sprite = Sprite::from_string("win-lose".into(), str);
    sprite.layer = 1;
    sprite.translation = Vector2D::new(0, GRID_HEIGHT + 1);

    engine.sprites.insert("win-lose".into(), sprite);
}





