use crate::game::{Game, Player};
use crate::hex::HexCoord;
use crossterm::{
    cursor,
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub struct GameUI {
    // UI state if needed
}

impl GameUI {
    pub fn new() -> Self {
        GameUI {}
    }

    pub fn display_welcome(&self) {
        self.clear_screen();
        println!("╔═══════════════════════════════════════════╗");
        println!("║            NEXUS - Hex Strategy           ║");
        println!("╠═══════════════════════════════════════════╣");
        println!("║  Rules:                                   ║");
        println!("║  • RED (You): Connect North ↔ South      ║");
        println!("║  • BLUE (AI): Connect East ↔ West        ║");
        println!("║  • Place one piece per turn               ║");
        println!("║  • First to connect wins!                 ║");
        println!("║                                           ║");
        println!("║  Input: Type coordinates like 'e5'        ║");
        println!("║  Quit: Type 'q' or 'quit'                 ║");
        println!("╚═══════════════════════════════════════════╝");
        println!();
    }

    pub fn display_board(&self, game: &Game) {
        println!();
        self.print_column_labels(game.size());

        for r in 0..game.size() {
            // Print row label
            print!("{:2} ", r + 1);

            // Add indentation for hex offset
            for _ in 0..r {
                print!(" ");
            }

            // Print hex cells
            for q in 0..game.size() {
                let pos = HexCoord::new(q, r);
                self.print_hex_cell(game, pos);
            }

            println!();
        }

        println!();
        println!("Move #{} | Current player: {}",
                 game.move_count() + 1,
                 self.player_name(game.current_player()));
        println!();
    }

    fn print_column_labels(&self, size: i32) {
        print!("   ");
        for q in 0..size {
            print!(" {}", self.column_label(q));
        }
        println!();
    }

    fn column_label(&self, q: i32) -> char {
        ((b'a' + q as u8) as char).to_ascii_uppercase()
    }

    fn print_hex_cell(&self, game: &Game, pos: HexCoord) {
        match game.get(pos) {
            Some(Player::Red) => {
                print!("{}", SetForegroundColor(Color::Red));
                print!(" ●");
                print!("{}", ResetColor);
            }
            Some(Player::Blue) => {
                print!("{}", SetForegroundColor(Color::Blue));
                print!(" ●");
                print!("{}", ResetColor);
            }
            None => {
                print!(" ·");
            }
        }
    }

    fn player_name(&self, player: Player) -> &str {
        match player {
            Player::Red => "RED (You)",
            Player::Blue => "BLUE (AI)",
        }
    }

    pub fn get_player_move(&self, game: &Game) -> Option<HexCoord> {
        loop {
            print!("Your move (e.g., e5): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error reading input. Try again.");
                continue;
            }

            let input = input.trim().to_lowercase();

            if input == "q" || input == "quit" {
                return None;
            }

            match self.parse_move(&input, game.size()) {
                Ok(pos) => {
                    if game.is_valid_move(pos) {
                        return Some(pos);
                    } else {
                        println!("Invalid move! That position is already occupied.");
                    }
                }
                Err(e) => {
                    println!("Invalid input: {}. Try again (e.g., e5).", e);
                }
            }
        }
    }

    fn parse_move(&self, input: &str, size: i32) -> Result<HexCoord, String> {
        if input.len() < 2 {
            return Err("Input too short".to_string());
        }

        let col_char = input.chars().next().unwrap();
        let row_str = &input[1..];

        if !col_char.is_ascii_alphabetic() {
            return Err("Column must be a letter".to_string());
        }

        let q = (col_char.to_ascii_lowercase() as u8 - b'a') as i32;
        let r = row_str.parse::<i32>()
            .map_err(|_| "Invalid row number".to_string())?
            - 1;

        if q < 0 || q >= size || r < 0 || r >= size {
            return Err(format!("Position out of bounds (board is {}x{})", size, size));
        }

        Ok(HexCoord::new(q, r))
    }

    pub fn display_thinking(&self) {
        println!("AI is thinking...");
    }

    pub fn display_ai_move(&self, pos: HexCoord) {
        println!("AI played: {}{}",
                 self.column_label(pos.q),
                 pos.r + 1);
    }

    pub fn display_error(&self, msg: &str) {
        println!("{}", SetForegroundColor(Color::Red));
        println!("Error: {}", msg);
        println!("{}", ResetColor);
    }

    pub fn display_winner(&self, winner: Option<Player>) {
        println!();
        println!("═══════════════════════════════════════");
        match winner {
            Some(Player::Red) => {
                println!("{}", SetForegroundColor(Color::Red));
                println!("    🎉 YOU WIN! Congratulations! 🎉");
                println!("{}", ResetColor);
            }
            Some(Player::Blue) => {
                println!("{}", SetForegroundColor(Color::Blue));
                println!("    AI WINS! Better luck next time!");
                println!("{}", ResetColor);
            }
            None => {
                println!("    Game ended in a draw!");
            }
        }
        println!("═══════════════════════════════════════");
        println!();
    }

    pub fn display_goodbye(&self) {
        println!();
        println!("Thanks for playing Nexus! Goodbye!");
    }

    fn clear_screen(&self) {
        let _ = execute!(
            io::stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        );
    }
}

impl Default for GameUI {
    fn default() -> Self {
        Self::new()
    }
}
