mod game;
mod entities;
mod weapons;
mod enemies;
mod powerups;
mod particles;
mod collision;
mod rendering;
mod audio;
mod state;
mod waves;
mod score;

use macroquad::prelude::*;
use game::Game;

/// Window configuration for the game
fn window_conf() -> Conf {
    Conf {
        window_title: "SPACE SHOOTER - Competition Edition".to_owned(),
        window_width: 800,
        window_height: 900,
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Initialize game
    let mut game = Game::new();

    // Main game loop
    loop {
        // Handle input and update
        game.handle_input();
        game.update(get_frame_time());

        // Render
        clear_background(BLACK);
        game.render();

        // Display FPS in debug builds
        #[cfg(debug_assertions)]
        {
            draw_text(
                &format!("FPS: {}", get_fps()),
                10.0,
                20.0,
                20.0,
                GREEN,
            );
        }

        next_frame().await;
    }
}
