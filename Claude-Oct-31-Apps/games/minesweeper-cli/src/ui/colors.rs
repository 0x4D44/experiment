//! ANSI color definitions

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Reset,
    Blue,
    Green,
    Red,
    DarkBlue,
    Magenta,
    Cyan,
    Black,
    White,
    Gray,
    Yellow,
}

impl Color {
    /// Get ANSI color code for this color
    pub fn code(&self) -> &'static str {
        match self {
            Color::Reset => "\x1b[0m",
            Color::Blue => "\x1b[34m",
            Color::Green => "\x1b[32m",
            Color::Red => "\x1b[31m",
            Color::DarkBlue => "\x1b[44m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::Black => "\x1b[30m",
            Color::White => "\x1b[37m",
            Color::Gray => "\x1b[90m",
            Color::Yellow => "\x1b[33m",
        }
    }

    /// Get color code for a mine number (1-8)
    pub fn for_mine_count(count: u8) -> &'static str {
        match count {
            1 => "\x1b[34m", // Blue
            2 => "\x1b[32m", // Green
            3 => "\x1b[31m", // Red
            4 => "\x1b[34m", // Dark blue
            5 => "\x1b[35m", // Magenta
            6 => "\x1b[36m", // Cyan
            7 => "\x1b[30m", // Black
            8 => "\x1b[90m", // Gray
            _ => "\x1b[37m", // White
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}
