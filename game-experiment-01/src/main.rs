mod game_state;
mod parser;
mod world;
mod memory;
mod evidence;
mod commands;

use std::io::{self, Write};
use game_state::GameState;
use parser::Parser;

fn main() {
    println!("=== THE MEMORY DETECTIVE ===");
    println!("A Text Adventure Mystery\n");
    println!("Type 'help' for commands or 'quit' to exit.\n");

    let mut game_state = GameState::new();
    let parser = Parser::new();

    // Show initial scene
    game_state.look();

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Please try again.");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Parse and execute command
        match parser.parse(input) {
            Ok(command) => {
                let result = game_state.execute_command(command);
                match result {
                    Ok(should_quit) => {
                        if should_quit {
                            println!("\nThank you for playing The Memory Detective!");
                            break;
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(e) => println!("I don't understand. {}", e),
        }

        // Check for game over conditions
        if game_state.is_game_over() {
            game_state.show_ending();
            break;
        }
    }
}
