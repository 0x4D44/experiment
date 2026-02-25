use flux::levels::LevelPack;
use flux::ui::{Command, Direction, InputHandler, Renderer};
use flux::{Grid, Position};
use std::io;

struct GameState {
    level_pack: LevelPack,
    current_level_id: usize,
    grid: Grid,
    cursor: Position,
    moves: usize,
    won: bool,
}

impl GameState {
    fn new() -> Self {
        let level_pack = LevelPack::main_campaign();
        let level = level_pack.get_level(0).expect("Level 0 should exist");
        let grid = level.create_grid();

        GameState {
            level_pack,
            current_level_id: 0,
            grid,
            cursor: Position::new(0, 0),
            moves: 0,
            won: false,
        }
    }

    fn current_level(&self) -> &flux::levels::Level {
        self.level_pack
            .get_level(self.current_level_id)
            .expect("Current level should exist")
    }

    fn move_cursor(&mut self, direction: Direction) {
        let level = self.current_level();
        match direction {
            Direction::Up => {
                if self.cursor.row > 0 {
                    self.cursor.row -= 1;
                }
            }
            Direction::Down => {
                if self.cursor.row < level.height - 1 {
                    self.cursor.row += 1;
                }
            }
            Direction::Left => {
                if self.cursor.col > 0 {
                    self.cursor.col -= 1;
                }
            }
            Direction::Right => {
                if self.cursor.col < level.width - 1 {
                    self.cursor.col += 1;
                }
            }
        }
    }

    fn click(&mut self) {
        if !self.won {
            self.grid.click(self.cursor);
            self.moves += 1;

            let level = self.current_level();
            if self.grid.matches_target(&level.target_state) {
                self.won = true;
            }
        }
    }

    fn reset(&mut self) {
        let level = self.current_level();
        self.grid = level.create_grid();
        self.cursor = Position::new(0, 0);
        self.moves = 0;
        self.won = false;
    }

    fn next_level(&mut self) -> bool {
        if self.current_level_id < self.level_pack.len() - 1 {
            self.current_level_id += 1;
            self.reset();
            true
        } else {
            false
        }
    }

    fn prev_level(&mut self) {
        if self.current_level_id > 0 {
            self.current_level_id -= 1;
            self.reset();
        }
    }
}

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;
    let mut game_state = GameState::new();
    let mut game_complete = false;

    loop {
        if game_complete {
            renderer.show_game_complete()?;
        } else if game_state.won {
            renderer.show_win_screen(game_state.current_level_id, game_state.moves)?;
        } else {
            renderer.render(
                game_state.current_level(),
                &game_state.grid,
                game_state.cursor,
                game_state.moves,
            )?;
        }

        if let Some(command) = InputHandler::read_command()? {
            match command {
                Command::Move(dir) => game_state.move_cursor(dir),
                Command::Click => game_state.click(),
                Command::Reset => game_state.reset(),
                Command::NextLevel => {
                    if !game_state.next_level() {
                        game_complete = true;
                    }
                }
                Command::PrevLevel => game_state.prev_level(),
                Command::Quit => break,
            }
        }
    }

    renderer.cleanup()?;
    Ok(())
}
