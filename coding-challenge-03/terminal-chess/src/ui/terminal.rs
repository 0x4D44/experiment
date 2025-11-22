use crate::chess::{Board, Color, Game, GameState, Move, PieceType, Position};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color as CColor, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

pub struct TerminalUI;

impl TerminalUI {
    pub fn new() -> Self {
        TerminalUI
    }

    /// Display the main menu and get user choice
    pub fn show_menu(&self) -> io::Result<MenuChoice> {
        self.clear_screen()?;

        println!("\n╔════════════════════════════════════════╗");
        println!("║        TERMINAL CHESS GAME             ║");
        println!("╚════════════════════════════════════════╝\n");
        println!("  1. Player vs Player");
        println!("  2. Player vs AI");
        println!("  3. Load Game");
        println!("  4. Quit\n");
        print!("Enter your choice (1-4): ");
        io::stdout().flush()?;

        loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('1') => return Ok(MenuChoice::PlayerVsPlayer),
                    KeyCode::Char('2') => return Ok(MenuChoice::PlayerVsAI),
                    KeyCode::Char('3') => return Ok(MenuChoice::LoadGame),
                    KeyCode::Char('4') | KeyCode::Char('q') => return Ok(MenuChoice::Quit),
                    _ => {}
                }
            }
        }
    }

    /// Get AI difficulty level
    pub fn get_difficulty(&self) -> io::Result<u8> {
        self.clear_screen()?;

        println!("\n╔════════════════════════════════════════╗");
        println!("║        SELECT AI DIFFICULTY            ║");
        println!("╚════════════════════════════════════════╝\n");
        println!("  1. Easy (Depth 1)");
        println!("  2. Medium (Depth 2)");
        println!("  3. Hard (Depth 3)");
        println!("  4. Expert (Depth 4)\n");
        print!("Enter difficulty (1-4): ");
        io::stdout().flush()?;

        loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('1') => return Ok(1),
                    KeyCode::Char('2') => return Ok(2),
                    KeyCode::Char('3') => return Ok(3),
                    KeyCode::Char('4') => return Ok(4),
                    _ => {}
                }
            }
        }
    }

    /// Display the game board
    pub fn display_game(&self, game: &Game, last_move: Option<Move>) -> io::Result<()> {
        self.clear_screen()?;

        println!("\n╔════════════════════════════════════════╗");
        println!("║        TERMINAL CHESS GAME             ║");
        println!("╚════════════════════════════════════════╝\n");

        // Display captured pieces
        let (white_captured, black_captured) = game.get_captured_pieces();

        print!("  Black captured: ");
        for piece in &white_captured {
            print!("{} ", piece.to_unicode());
        }
        println!("\n");

        // Display board
        self.display_board(&game.board, last_move)?;

        print!("\n  White captured: ");
        for piece in &black_captured {
            print!("{} ", piece.to_unicode());
        }
        println!();

        // Display game state
        println!("\n  Current player: {}", if game.current_player == Color::White { "White" } else { "Black" });

        match game.state {
            GameState::Check => println!("  Status: CHECK!"),
            GameState::Checkmate(winner) => {
                println!("  Status: CHECKMATE! {} wins!", if winner == Color::White { "White" } else { "Black" });
            }
            GameState::Stalemate => println!("  Status: STALEMATE! Game is a draw."),
            GameState::Draw => println!("  Status: DRAW (50-move rule)"),
            GameState::Playing => println!("  Status: Playing"),
        }

        println!("\n  Commands: [move] e2e4, [u]ndo, [s]ave, [q]uit");

        Ok(())
    }

    /// Display the chess board with colors
    fn display_board(&self, board: &Board, last_move: Option<Move>) -> io::Result<()> {
        let light_square = CColor::Rgb { r: 240, g: 217, b: 181 };
        let dark_square = CColor::Rgb { r: 181, g: 136, b: 99 };
        let highlight_square = CColor::Rgb { r: 170, g: 162, b: 58 };
        let white_piece_color = CColor::White;
        let black_piece_color = CColor::Black;

        // Print column labels
        print!("     ");
        for col in 0..8 {
            print!(" {} ", (b'a' + col as u8) as char);
        }
        println!();

        // Print top border
        println!("    ┌────────────────────────┐");

        // Print board rows (from top to bottom, which is row 7 to 0)
        for row in (0..8).rev() {
            print!("  {} │", row + 1);

            for col in 0..8 {
                let pos = Position::new(row, col).unwrap();
                let is_light = (row + col) % 2 == 0;

                // Check if this square is part of the last move
                let is_highlighted = last_move.is_some_and(|m| m.from == pos || m.to == pos);

                let bg_color = if is_highlighted {
                    highlight_square
                } else if is_light {
                    light_square
                } else {
                    dark_square
                };

                execute!(io::stdout(), SetBackgroundColor(bg_color))?;

                if let Some(piece) = board.get_piece(pos) {
                    let fg_color = if piece.color == Color::White {
                        white_piece_color
                    } else {
                        black_piece_color
                    };
                    execute!(io::stdout(), SetForegroundColor(fg_color))?;
                    print!(" {} ", piece.to_unicode());
                } else {
                    print!("   ");
                }

                execute!(io::stdout(), ResetColor)?;
            }

            println!("│ {}", row + 1);
        }

        // Print bottom border
        println!("    └────────────────────────┘");

        // Print column labels again
        print!("     ");
        for col in 0..8 {
            print!(" {} ", (b'a' + col as u8) as char);
        }
        println!("\n");

        Ok(())
    }

    /// Get a move from the user
    pub fn get_move(&self, prompt: &str) -> io::Result<String> {
        print!("  {}: ", prompt);
        io::stdout().flush()?;

        let mut input = String::new();

        loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                        io::stdout().flush()?;
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            execute!(io::stdout(), cursor::MoveLeft(1), Print(' '), cursor::MoveLeft(1))?;
                        }
                    }
                    KeyCode::Enter => {
                        println!();
                        return Ok(input.trim().to_lowercase());
                    }
                    _ => {}
                }
            }
        }
    }

    /// Get filename for save/load
    pub fn get_filename(&self, prompt: &str) -> io::Result<String> {
        self.get_move(prompt)
    }

    /// Display a message
    pub fn show_message(&self, message: &str) -> io::Result<()> {
        println!("\n  {}", message);
        println!("  Press any key to continue...");
        io::stdout().flush()?;

        event::read()?;
        Ok(())
    }

    /// Display a thinking message
    pub fn show_thinking(&self) -> io::Result<()> {
        print!("\n  AI is thinking");
        io::stdout().flush()?;

        for _ in 0..3 {
            std::thread::sleep(std::time::Duration::from_millis(300));
            print!(".");
            io::stdout().flush()?;
        }
        println!();

        Ok(())
    }

    /// Clear the screen
    fn clear_screen(&self) -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        Ok(())
    }

    /// Enable raw mode for terminal
    pub fn enable_raw_mode(&self) -> io::Result<()> {
        terminal::enable_raw_mode()
    }

    /// Disable raw mode for terminal
    pub fn disable_raw_mode(&self) -> io::Result<()> {
        terminal::disable_raw_mode()
    }

    /// Parse a move from algebraic notation
    pub fn parse_move(&self, input: &str, game: &Game) -> Result<Move, String> {
        // Handle special commands
        if input.starts_with('u') {
            return Err("UNDO".to_string());
        }
        if input.starts_with('s') {
            return Err("SAVE".to_string());
        }
        if input.starts_with('q') {
            return Err("QUIT".to_string());
        }

        // Parse algebraic notation (e.g., "e2e4" or "e7e8q" for promotion)
        if input.len() < 4 {
            return Err("Invalid move format. Use algebraic notation (e.g., e2e4)".to_string());
        }

        let from = Position::from_algebraic(&input[0..2])
            .ok_or_else(|| "Invalid source position".to_string())?;

        let to = Position::from_algebraic(&input[2..4])
            .ok_or_else(|| "Invalid destination position".to_string())?;

        let piece = game.board.get_piece(from)
            .ok_or_else(|| "No piece at source position".to_string())?;

        if piece.color != game.current_player {
            return Err("Not your piece".to_string());
        }

        let captured = game.board.get_piece(to);

        // Check for promotion
        let mov = if input.len() >= 5 {
            let promote_char = input.chars().nth(4).unwrap();
            let promote_to = match promote_char {
                'q' | 'Q' => PieceType::Queen,
                'r' | 'R' => PieceType::Rook,
                'b' | 'B' => PieceType::Bishop,
                'n' | 'N' => PieceType::Knight,
                _ => return Err("Invalid promotion piece. Use q, r, b, or n".to_string()),
            };
            Move::promotion(from, to, piece, promote_to, captured)
        } else {
            // Check if it's a special move by looking at legal moves
            let legal_moves = game.get_legal_moves();
            if let Some(legal_move) = legal_moves.iter().find(|m| m.from == from && m.to == to) {
                *legal_move
            } else {
                // Just a normal move attempt
                if let Some(cap) = captured {
                    Move::with_capture(from, to, piece, cap)
                } else {
                    Move::new(from, to, piece)
                }
            }
        };

        Ok(mov)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuChoice {
    PlayerVsPlayer,
    PlayerVsAI,
    LoadGame,
    Quit,
}

impl Default for TerminalUI {
    fn default() -> Self {
        Self::new()
    }
}
