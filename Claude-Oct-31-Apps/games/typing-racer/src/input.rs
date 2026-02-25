// Input buffer and handling

use crate::config::MAX_INPUT_LENGTH;

pub struct InputBuffer {
    buffer: String,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(MAX_INPUT_LENGTH),
        }
    }

    /// Add a character to the input buffer (converts to uppercase)
    pub fn add_char(&mut self, c: char) {
        if self.buffer.len() < MAX_INPUT_LENGTH && c.is_alphabetic() {
            self.buffer.push(c.to_uppercase().to_string().chars().next().unwrap());
        }
    }

    /// Remove the last character from the buffer
    pub fn backspace(&mut self) {
        self.buffer.pop();
    }

    /// Clear the entire buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get the current buffer text
    pub fn get_text(&self) -> String {
        self.buffer.clone()
    }

    /// Get the buffer as a string slice
    pub fn as_str(&self) -> &str {
        &self.buffer
    }

    /// Get the length of the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Check if buffer is full
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= MAX_INPUT_LENGTH
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for InputBuffer {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_character() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('a');
        assert_eq!(buffer.get_text(), "A");
    }

    #[test]
    fn test_add_multiple_characters() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('h');
        buffer.add_char('e');
        buffer.add_char('l');
        buffer.add_char('l');
        buffer.add_char('o');
        assert_eq!(buffer.get_text(), "HELLO");
    }

    #[test]
    fn test_case_conversion() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('A');
        assert_eq!(buffer.get_text(), "A");
        buffer.clear();
        buffer.add_char('z');
        assert_eq!(buffer.get_text(), "Z");
    }

    #[test]
    fn test_backspace() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('h');
        buffer.add_char('i');
        buffer.backspace();
        assert_eq!(buffer.get_text(), "H");
    }

    #[test]
    fn test_backspace_empty() {
        let mut buffer = InputBuffer::new();
        buffer.backspace();
        assert_eq!(buffer.get_text(), "");
    }

    #[test]
    fn test_clear() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('h');
        buffer.add_char('i');
        buffer.clear();
        assert_eq!(buffer.get_text(), "");
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_non_alphabetic_ignored() {
        let mut buffer = InputBuffer::new();
        buffer.add_char('a');
        buffer.add_char('1');
        buffer.add_char('b');
        // Numbers and special chars should be ignored
        assert_eq!(buffer.get_text(), "AB");
    }

    #[test]
    fn test_length() {
        let mut buffer = InputBuffer::new();
        assert_eq!(buffer.len(), 0);
        buffer.add_char('h');
        assert_eq!(buffer.len(), 1);
        buffer.add_char('i');
        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut buffer = InputBuffer::new();
        assert!(buffer.is_empty());
        buffer.add_char('a');
        assert!(!buffer.is_empty());
    }
}
