mod ai;
mod board;
mod game;
mod ui;

use game::{show_menu, Game};
use ui::UI;

fn main() {
    // Initialize terminal
    if let Err(e) = UI::init() {
        eprintln!("Failed to initialize terminal: {}", e);
        std::process::exit(1);
    }

    // Ensure cleanup happens on exit
    let cleanup = || {
        let _ = UI::cleanup();
    };

    // Set up panic handler
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = UI::cleanup();
        original_hook(panic_info);
    }));

    // Show menu and start game
    let result = run_game();

    cleanup();

    if let Err(e) = result {
        eprintln!("Game error: {}", e);
        std::process::exit(1);
    }
}

fn run_game() -> std::io::Result<()> {
    // Show main menu
    let mode = show_menu()?;

    // Create and run game
    let mut game = Game::new(mode);
    game.run()?;

    Ok(())
}
