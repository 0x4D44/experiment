//! Utility functions

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Check if coordinates are in bounds
pub fn in_bounds(x: usize, y: usize, width: usize, height: usize) -> bool {
    x < width && y < height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_below_min() {
        assert_eq!(clamp(5, 10, 20), 10);
    }

    #[test]
    fn test_clamp_above_max() {
        assert_eq!(clamp(25, 10, 20), 20);
    }

    #[test]
    fn test_clamp_within_range() {
        assert_eq!(clamp(15, 10, 20), 15);
    }

    #[test]
    fn test_in_bounds_valid() {
        assert!(in_bounds(5, 5, 10, 10));
    }

    #[test]
    fn test_in_bounds_at_edge() {
        assert!(in_bounds(9, 9, 10, 10));
    }

    #[test]
    fn test_in_bounds_outside() {
        assert!(!in_bounds(10, 10, 10, 10));
    }
}
