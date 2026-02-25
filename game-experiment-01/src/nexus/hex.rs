/// Hexagonal coordinate system using axial coordinates (q, r)
/// This system is perfect for hex grids where we need to track neighbors and distances
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32,  // column
    pub r: i32,  // row
}

impl HexCoord {
    pub fn new(q: i32, r: i32) -> Self {
        HexCoord { q, r }
    }

    /// Get the six neighboring hexes (NE, E, SE, SW, W, NW)
    pub fn neighbors(&self) -> [HexCoord; 6] {
        [
            HexCoord::new(self.q + 1, self.r),     // E
            HexCoord::new(self.q + 1, self.r - 1), // NE
            HexCoord::new(self.q, self.r - 1),     // NW
            HexCoord::new(self.q - 1, self.r),     // W
            HexCoord::new(self.q - 1, self.r + 1), // SW
            HexCoord::new(self.q, self.r + 1),     // SE
        ]
    }

    /// Calculate distance between two hexes (Manhattan distance in cube coordinates)
    pub fn distance(&self, other: &HexCoord) -> u32 {
        let dq = (self.q - other.q).abs();
        let dr = (self.r - other.r).abs();
        let ds = ((-self.q - self.r) - (-other.q - other.r)).abs();
        ((dq + dr + ds) / 2) as u32
    }

    /// Check if coordinate is within board bounds
    pub fn is_valid(&self, size: i32) -> bool {
        self.q >= 0 && self.q < size && self.r >= 0 && self.r < size
    }

    /// Check if this hex is on the north edge
    pub fn is_north_edge(&self) -> bool {
        self.r == 0
    }

    /// Check if this hex is on the south edge
    pub fn is_south_edge(&self, size: i32) -> bool {
        self.r == size - 1
    }

    /// Check if this hex is on the west edge
    pub fn is_west_edge(&self) -> bool {
        self.q == 0
    }

    /// Check if this hex is on the east edge
    pub fn is_east_edge(&self, size: i32) -> bool {
        self.q == size - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_creation() {
        let hex = HexCoord::new(3, 4);
        assert_eq!(hex.q, 3);
        assert_eq!(hex.r, 4);
    }

    #[test]
    fn test_hex_neighbors() {
        let hex = HexCoord::new(5, 5);
        let neighbors = hex.neighbors();

        assert_eq!(neighbors[0], HexCoord::new(6, 5));  // E
        assert_eq!(neighbors[1], HexCoord::new(6, 4));  // NE
        assert_eq!(neighbors[2], HexCoord::new(5, 4));  // NW
        assert_eq!(neighbors[3], HexCoord::new(4, 5));  // W
        assert_eq!(neighbors[4], HexCoord::new(4, 6));  // SW
        assert_eq!(neighbors[5], HexCoord::new(5, 6));  // SE
    }

    #[test]
    fn test_hex_distance() {
        let hex1 = HexCoord::new(0, 0);
        let hex2 = HexCoord::new(3, 0);
        assert_eq!(hex1.distance(&hex2), 3);

        let hex3 = HexCoord::new(0, 3);
        assert_eq!(hex1.distance(&hex3), 3);

        let hex4 = HexCoord::new(2, 2);
        assert_eq!(hex1.distance(&hex4), 4);
    }

    #[test]
    fn test_hex_validity() {
        let hex1 = HexCoord::new(5, 5);
        assert!(hex1.is_valid(11));
        assert!(!hex1.is_valid(5));

        let hex2 = HexCoord::new(-1, 5);
        assert!(!hex2.is_valid(11));

        let hex3 = HexCoord::new(11, 5);
        assert!(!hex3.is_valid(11));
    }

    #[test]
    fn test_edge_detection() {
        let size = 11;

        let north = HexCoord::new(5, 0);
        assert!(north.is_north_edge());
        assert!(!north.is_south_edge(size));

        let south = HexCoord::new(5, 10);
        assert!(!south.is_north_edge());
        assert!(south.is_south_edge(size));

        let west = HexCoord::new(0, 5);
        assert!(west.is_west_edge());
        assert!(!west.is_east_edge(size));

        let east = HexCoord::new(10, 5);
        assert!(!east.is_west_edge());
        assert!(east.is_east_edge(size));

        let center = HexCoord::new(5, 5);
        assert!(!center.is_north_edge());
        assert!(!center.is_south_edge(size));
        assert!(!center.is_west_edge());
        assert!(!center.is_east_edge(size));
    }
}
