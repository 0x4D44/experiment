mod ai;
mod combat;
mod entity;
mod fov;
mod game;
mod highscore;
mod items;
mod map;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let result = run_game();

    // Cleanup terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run_game() -> io::Result<()> {
    let mut game = game::Game::new();

    loop {
        // Render the game
        ui::render(&game)?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                let action = handle_input(&game, key_event);

                match action {
                    GameAction::Quit => break,
                    GameAction::Move(dx, dy) => {
                        if game.is_player_turn() {
                            game.try_move_player(dx, dy);
                        }
                    }
                    GameAction::PickupItem => {
                        if game.is_player_turn() {
                            game.pickup_item();
                        }
                    }
                    GameAction::UseItem(index) => {
                        if game.is_player_turn() {
                            game.use_item(index);
                        }
                    }
                    GameAction::DropItem(index) => {
                        if game.is_player_turn() {
                            game.drop_item(index);
                        }
                    }
                    GameAction::Descend => {
                        if game.is_player_turn() {
                            game.descend_stairs();
                        }
                    }
                    GameAction::NewGame => {
                        game = game::Game::new();
                    }
                    GameAction::None => {}
                }
            }
        }

        // Process AI turns
        if game.is_player_turn() {
            // Player's turn is over, let enemies act
        } else {
            game.process_enemy_turns();
        }
    }

    Ok(())
}

enum GameAction {
    None,
    Quit,
    Move(i32, i32),
    PickupItem,
    UseItem(usize),
    DropItem(usize),
    Descend,
    NewGame,
}

fn handle_input(game: &game::Game, key: KeyEvent) -> GameAction {
    if game.is_game_over() {
        return match key.code {
            KeyCode::Char('q') | KeyCode::Esc => GameAction::Quit,
            KeyCode::Char('n') => GameAction::NewGame,
            _ => GameAction::None,
        };
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => GameAction::Quit,

        // Movement (Vi keys + Arrow keys)
        KeyCode::Up | KeyCode::Char('k') => GameAction::Move(0, -1),
        KeyCode::Down | KeyCode::Char('j') => GameAction::Move(0, 1),
        KeyCode::Left | KeyCode::Char('h') => GameAction::Move(-1, 0),
        KeyCode::Right | KeyCode::Char('l') => GameAction::Move(1, 0),
        KeyCode::Char('y') => GameAction::Move(-1, -1),
        KeyCode::Char('u') => GameAction::Move(1, -1),
        KeyCode::Char('b') => GameAction::Move(-1, 1),
        KeyCode::Char('n') => GameAction::Move(1, 1),

        // Actions
        KeyCode::Char('g') | KeyCode::Char(',') => GameAction::PickupItem,
        KeyCode::Char('>') => GameAction::Descend,

        // Inventory (numbers 1-9)
        KeyCode::Char(c @ '1'..='9') => {
            let index = c.to_digit(10).unwrap() as usize - 1;
            GameAction::UseItem(index)
        }

        KeyCode::Char('d') => {
            // Drop first item if inventory not empty
            if !game.player.inventory.is_empty() {
                GameAction::DropItem(0)
            } else {
                GameAction::None
            }
        }

        _ => GameAction::None,
    }
}
