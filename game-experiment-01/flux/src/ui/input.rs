use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Move(Direction),
    Click,
    Reset,
    NextLevel,
    PrevLevel,
    Quit,
}

pub struct InputHandler;

impl InputHandler {
    pub fn read_command() -> io::Result<Option<Command>> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                return Ok(match code {
                    KeyCode::Up
                    | KeyCode::Char('w')
                    | KeyCode::Char('W')
                    | KeyCode::Char('k')
                    | KeyCode::Char('K') => Some(Command::Move(Direction::Up)),
                    KeyCode::Down
                    | KeyCode::Char('s')
                    | KeyCode::Char('S')
                    | KeyCode::Char('j')
                    | KeyCode::Char('J') => Some(Command::Move(Direction::Down)),
                    KeyCode::Left
                    | KeyCode::Char('a')
                    | KeyCode::Char('A')
                    | KeyCode::Char('h')
                    | KeyCode::Char('H') => Some(Command::Move(Direction::Left)),
                    KeyCode::Right
                    | KeyCode::Char('d')
                    | KeyCode::Char('D')
                    | KeyCode::Char('l')
                    | KeyCode::Char('L') => Some(Command::Move(Direction::Right)),
                    KeyCode::Enter | KeyCode::Char(' ') => Some(Command::Click),
                    KeyCode::Char('r') | KeyCode::Char('R') => Some(Command::Reset),
                    KeyCode::Char('n') | KeyCode::Char('N') => Some(Command::NextLevel),
                    KeyCode::Char('p') | KeyCode::Char('P') => Some(Command::PrevLevel),
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Command::Quit),
                    _ => None,
                });
            }
        }
        Ok(None)
    }
}
