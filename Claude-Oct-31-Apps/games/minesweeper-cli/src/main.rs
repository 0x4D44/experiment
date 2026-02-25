//! Minesweeper CLI - Main entry point
//!
//! A fully-featured Minesweeper game for the command line in Rust.

use minesweeper_cli::*;
use std::io;

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║      MINESWEEPER CLI - MAIN MENU      ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("Select difficulty:");
    println!("1. Beginner (9×9, 10 mines)");
    println!("2. Intermediate (16×16, 40 mines)");
    println!("3. Expert (30×16, 99 mines)");
    println!("4. Custom");
    println!("5. View Statistics");
    println!("6. Quit\n");

    print!("Enter choice (1-6): ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => play_game(Difficulty::beginner()),
        "2" => play_game(Difficulty::intermediate()),
        "3" => play_game(Difficulty::expert()),
        "4" => {
            if let Some(custom) = create_custom_game() {
                play_game(custom);
            }
        }
        "5" => show_statistics(),
        "6" => println!("Thanks for playing!"),
        _ => println!("Invalid choice"),
    }
}

/// Play a single game with given difficulty
fn play_game(difficulty: Difficulty) {
    println!("\n{}", difficulty);
    println!("Starting new game...\n");

    let mut game = Game::new(difficulty);
    let mut cursor_x = 4;
    let mut cursor_y = 4;

    println!("Click to start (enter coordinates x,y): ");
    print!("> ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Some((x, y)) = parse_coordinates(&input) {
        game.handle_first_click(x, y);

        // Simple game loop
        loop {
            // Render board
            println!("\x1b[2J\x1b[H"); // Clear screen
            println!("{}", ui::render_board(&game, cursor_x, cursor_y));

            if !matches!(game.status(), GameStatus::Playing) {
                break;
            }

            // Get input
            print!("> ");
            io::Write::flush(&mut io::stdout()).unwrap();

            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).unwrap();

            match cmd.trim() {
                "up" | "w" => cursor_y = cursor_y.saturating_sub(1),
                "down" | "s" => cursor_y = (cursor_y + 1).min(game.board().height() - 1),
                "left" | "a" => cursor_x = cursor_x.saturating_sub(1),
                "right" | "d" => cursor_x = (cursor_x + 1).min(game.board().width() - 1),
                "space" | "click" => game.reveal(cursor_x, cursor_y),
                "f" | "flag" => game.toggle_flag(cursor_x, cursor_y),
                "c" | "chord" => game.chord(cursor_x, cursor_y),
                "r" | "restart" => {
                    let mines = game.board().mines_count();
                    let width = game.board().width();
                    let height = game.board().height();
                    return play_game(Difficulty::custom(width, height, mines).unwrap_or(Difficulty::beginner()));
                }
                "q" | "quit" => break,
                _ => {}
            }
        }

        // Show final result
        match game.status() {
            GameStatus::Won => {
                println!("\n🎉 You won! Time: {}", game.timer().formatted());
            }
            GameStatus::Lost { revealed_mine } => {
                println!("\n💥 Game over! Mine revealed at ({}, {})", revealed_mine.0, revealed_mine.1);
            }
            _ => {}
        }
    }
}

/// Parse coordinate input
fn parse_coordinates(input: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = input.trim().split(',').collect();
    if parts.len() == 2 {
        if let (Ok(x), Ok(y)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
            return Some((x, y));
        }
    }
    None
}

/// Create a custom game
fn create_custom_game() -> Option<Difficulty> {
    println!("\nCreate Custom Game");
    println!("Enter width (4-100): ");
    print!("> ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut width_str = String::new();
    io::stdin().read_line(&mut width_str).unwrap();
    let width: usize = width_str.trim().parse().ok()?;

    println!("Enter height (4-100): ");
    print!("> ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut height_str = String::new();
    io::stdin().read_line(&mut height_str).unwrap();
    let height: usize = height_str.trim().parse().ok()?;

    println!("Enter number of mines: ");
    print!("> ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut mines_str = String::new();
    io::stdin().read_line(&mut mines_str).unwrap();
    let mines: usize = mines_str.trim().parse().ok()?;

    match Difficulty::custom(width, height, mines) {
        Ok(diff) => {
            println!("Custom game created: {}", diff);
            Some(diff)
        }
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

/// Show game statistics
fn show_statistics() {
    println!("\n{}", statistics::Statistics::new().display());
}
