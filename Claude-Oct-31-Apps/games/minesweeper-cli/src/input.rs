//! Input handling module

/// Input event enumeration
#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    Move(Direction),
    Reveal,
    Flag,
    Chord,
    Undo,
    Redo,
    Hint,
    Restart,
    Save,
    Load,
    Quit,
}

/// Direction enumeration for movement
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_event_creation() {
        let event = InputEvent::Move(Direction::Up);
        assert_eq!(event, InputEvent::Move(Direction::Up));
    }

    #[test]
    fn test_direction_equality() {
        assert_eq!(Direction::Up, Direction::Up);
        assert_ne!(Direction::Up, Direction::Down);
    }
}
