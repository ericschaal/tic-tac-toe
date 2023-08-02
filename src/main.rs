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

const Y_GRID: usize = 2;
const Y_INSTRUCTIONS: usize = Y_GRID + GRID_HEIGHT + 1;
const Y_TURN: usize = 0;
const Y_FPS: usize = Y_INSTRUCTIONS + 2;

fn main() -> Result<()> {

    let grid = Sprite::from_file("grid", "sprites/grid")?
        .with_translation(Vector2D::new(0, Y_GRID))
        .with_layer(0);

    let instructions = Sprite::from_string("instructions", "Q: Quit, Space: Play, Arrows: Move")
        .with_translation(Vector2D::new(0, Y_INSTRUCTIONS))
        .with_layer(0);

    Engine::new(Window::new(50, 50))
        .set_fps(60)
        .with_sprite("grid", grid)
        .with_sprite("instructions", instructions)
        .with_logic(base_logic)
        .with_logic(game_movement)
        .with_logic(cursor_logic)
        .with_logic(markers_logic)
        .with_logic(win_lose_logic)
        .with_logic(fps_counter)
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
        let mut sprite = Sprite::from_string("cursor", cursor_str);
        let board = engine.get_sprite("grid").unwrap();
        let cursor_coords = game.cursor_position.to_frame_coordinates(&board.translation);
        let translate = Vector2D::new(cursor_coords.x, cursor_coords.y);

        sprite.translation = translate;
        sprite.layer = 1;

        if game.turn == Player::ONE {
            engine.remove_sprite("p2");
            engine.insert_sprite("p1", sprite);
        } else if game.turn == Player::TWO {
            engine.remove_sprite("p1");
            engine.insert_sprite("p2", sprite);
        }
    }
}

fn markers_logic(engine: &mut Engine<Game>, game: &mut Game) {
    for (x, col) in game.state.iter().enumerate() {
        for (y, cell) in col.iter().enumerate() {
            if *cell == Player::ONE || *cell == Player::TWO {
                let marker_str = if *cell == Player::ONE {"X"} else {"O"};
                let mut sprite = Sprite::from_string("marker", marker_str);
                let board = engine.get_sprite("grid").unwrap();
                let translate = BoardCoordinates::new(x, y).to_frame_coordinates(&board.translation);

                sprite.translation = Vector2D::new(translate.x, translate.y);
                sprite.layer = 1;

                engine.insert_sprite(format!("{}-{}", x, y).as_str(), sprite);
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

    engine.remove_sprite("win-lose");

    let mut sprite = Sprite::from_string("win-lose", str.as_str());
    sprite.layer = 1;
    sprite.translation = Vector2D::new(0, Y_TURN);

    engine.insert_sprite("win-lose", sprite);
}

fn fps_counter<T>(engine: &mut Engine<T>, _: &mut T) {
    if engine.delta.as_millis() > 0.0 as u128 {
        let fps = 1000_u128 / engine.delta.as_millis();
        let str = format!("FPS: {}", fps);

        engine.remove_sprite("fps");
        let mut sprite = Sprite::from_string("fps", str.as_str());
        sprite.layer = 1;
        sprite.translation = Vector2D::new(0, Y_FPS);
        engine.insert_sprite("fps", sprite);
    }
}





