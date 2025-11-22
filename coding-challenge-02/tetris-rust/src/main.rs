mod game;
mod pieces;
mod board;
mod particles;
mod storage;

use macroquad::prelude::*;
use game::Game;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 900;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris Champion - Coding Challenge Edition".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(Color::from_rgba(15, 15, 25, 255));

        game.update(get_frame_time());
        game.draw();

        next_frame().await;
    }
}
