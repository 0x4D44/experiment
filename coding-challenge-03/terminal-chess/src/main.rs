use terminal_chess::{ChessAI, Color, Game, GameMode, GameState, TerminalUI};
use terminal_chess::ui::terminal::MenuChoice;
use std::io;

fn main() -> io::Result<()> {
    let ui = TerminalUI::new();

    // Enable raw mode for better terminal control
    ui.enable_raw_mode()?;

    let result = run_game(&ui);

    // Disable raw mode before exiting
    ui.disable_raw_mode()?;

    result
}

fn run_game(ui: &TerminalUI) -> io::Result<()> {
    loop {
        let choice = ui.show_menu()?;

        match choice {
            MenuChoice::Quit => {
                println!("\nThanks for playing! Goodbye!\n");
                break;
            }
            MenuChoice::PlayerVsPlayer => {
                let mut game = Game::new(GameMode::PlayerVsPlayer, 0);
                play_game(ui, &mut game)?;
            }
            MenuChoice::PlayerVsAI => {
                let difficulty = ui.get_difficulty()?;
                let mut game = Game::new(GameMode::PlayerVsAI, difficulty);
                play_game(ui, &mut game)?;
            }
            MenuChoice::LoadGame => {
                let filename = ui.get_filename("Enter filename to load")?;
                match Game::load_from_file(&filename) {
                    Ok(mut game) => {
                        ui.show_message(&format!("Game loaded from {}", filename))?;
                        play_game(ui, &mut game)?;
                    }
                    Err(e) => {
                        ui.show_message(&format!("Failed to load game: {}", e))?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn play_game(ui: &TerminalUI, game: &mut Game) -> io::Result<()> {
    let mut last_move = None;

    loop {
        // Display the board
        ui.display_game(game, last_move)?;

        // Check if game is over
        if game.is_game_over() {
            match game.state {
                GameState::Checkmate(winner) => {
                    ui.show_message(&format!(
                        "Checkmate! {} wins!",
                        if winner == Color::White { "White" } else { "Black" }
                    ))?;
                }
                GameState::Stalemate => {
                    ui.show_message("Stalemate! The game is a draw.")?;
                }
                GameState::Draw => {
                    ui.show_message("Draw by 50-move rule.")?;
                }
                _ => {}
            }
            break;
        }

        // Get move based on game mode
        let mov_option = if game.mode == GameMode::PlayerVsAI && game.current_player == Color::Black {
            // AI's turn
            ui.show_thinking()?;
            let ai = ChessAI::new(game.ai_difficulty);
            ai.find_best_move(&game.board, Color::Black)
        } else {
            // Human's turn
            loop {
                let input = ui.get_move("Enter move")?;

                match ui.parse_move(&input, game) {
                    Ok(mov) => break Some(mov),
                    Err(e) => {
                        if e == "UNDO" {
                            // Undo move
                            if game.mode == GameMode::PlayerVsAI {
                                // Undo two moves in AI mode (player's and AI's)
                                if let Err(e) = game.undo_move() {
                                    ui.show_message(&format!("Cannot undo: {}", e))?;
                                } else if let Err(e) = game.undo_move() {
                                    // Only undo one move if we can't undo two
                                    ui.show_message(&format!("Only undid one move: {}", e))?;
                                }
                            } else {
                                // Undo one move in PvP mode
                                if let Err(e) = game.undo_move() {
                                    ui.show_message(&format!("Cannot undo: {}", e))?;
                                }
                            }
                            last_move = None;
                            break None;
                        } else if e == "SAVE" {
                            // Save game
                            let filename = ui.get_filename("Enter filename to save")?;
                            match game.save_to_file(&filename) {
                                Ok(_) => ui.show_message(&format!("Game saved to {}", filename))?,
                                Err(e) => ui.show_message(&format!("Failed to save: {}", e))?,
                            }
                            continue;
                        } else if e == "QUIT" {
                            return Ok(());
                        } else {
                            ui.show_message(&e)?;
                            continue;
                        }
                    }
                }
            }
        };

        // If undo was performed, skip move execution
        if let Some(mov) = mov_option {
            // Make the move
            match game.make_move(mov) {
                Ok(_) => {
                    last_move = Some(mov);
                }
                Err(e) => {
                    ui.show_message(&format!("Invalid move: {}", e))?;
                }
            }
        }
    }

    Ok(())
}
