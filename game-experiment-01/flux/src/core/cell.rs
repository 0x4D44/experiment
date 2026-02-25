/// Represents a cell's state in Z/4Z (integers modulo 4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CellState(u8);

impl CellState {
    /// Create a new cell state with the given value (0-3)
    /// Panics if value is not in 0-3
    pub fn new(value: u8) -> Self {
        assert!(value < 4, "CellState value must be 0-3, got {}", value);
        CellState(value)
    }

    /// Get the current value (0-3)
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Increment by 1 (mod 4)
    pub fn increment(&mut self) {
        self.0 = (self.0 + 1) % 4;
    }

    /// Increment by n (mod 4)
    pub fn increment_by(&mut self, n: u8) {
        self.0 = (self.0 + n) % 4;
    }

    /// Check if state is 0
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cell_state() {
        assert_eq!(CellState::new(0).value(), 0);
        assert_eq!(CellState::new(1).value(), 1);
        assert_eq!(CellState::new(2).value(), 2);
        assert_eq!(CellState::new(3).value(), 3);
    }

    #[test]
    #[should_panic(expected = "CellState value must be 0-3")]
    fn test_new_cell_state_invalid() {
        CellState::new(4);
    }

    #[test]
    fn test_default() {
        assert_eq!(CellState::default().value(), 0);
    }

    #[test]
    fn test_increment() {
        let mut cell = CellState::new(0);
        cell.increment();
        assert_eq!(cell.value(), 1);
        cell.increment();
        assert_eq!(cell.value(), 2);
        cell.increment();
        assert_eq!(cell.value(), 3);
        cell.increment();
        assert_eq!(cell.value(), 0); // Wraps around
    }

    #[test]
    fn test_increment_by() {
        let mut cell = CellState::new(0);
        cell.increment_by(2);
        assert_eq!(cell.value(), 2);

        let mut cell = CellState::new(3);
        cell.increment_by(1);
        assert_eq!(cell.value(), 0); // Wraps around

        let mut cell = CellState::new(1);
        cell.increment_by(5); // 5 mod 4 = 1, so 1+1=2
        assert_eq!(cell.value(), 2);
    }

    #[test]
    fn test_is_zero() {
        assert!(CellState::new(0).is_zero());
        assert!(!CellState::new(1).is_zero());
        assert!(!CellState::new(2).is_zero());
        assert!(!CellState::new(3).is_zero());
    }

    #[test]
    fn test_equality() {
        assert_eq!(CellState::new(0), CellState::new(0));
        assert_eq!(CellState::new(2), CellState::new(2));
        assert_ne!(CellState::new(0), CellState::new(1));
    }
}
