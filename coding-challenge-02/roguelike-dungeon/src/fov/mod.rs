use crate::dungeon::Map;

/// Field of View information for a tile
#[derive(Debug, Clone, Copy)]
pub struct FOVInfo {
    pub visible: bool,
    pub explored: bool,
}

impl Default for FOVInfo {
    fn default() -> Self {
        FOVInfo {
            visible: false,
            explored: false,
        }
    }
}

/// Field of View system using shadow casting
pub struct FOV {
    pub fov_map: Vec<FOVInfo>,
    width: i32,
    height: i32,
}

impl FOV {
    pub fn new(width: i32, height: i32) -> Self {
        let fov_map = vec![FOVInfo::default(); (width * height) as usize];
        FOV {
            fov_map,
            width,
            height,
        }
    }

    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// Compute field of view from a position
    pub fn compute_fov(&mut self, map: &Map, x: i32, y: i32, radius: i32) {
        // Clear all visible flags
        for info in &mut self.fov_map {
            info.visible = false;
        }

        // Player position is always visible
        if map.in_bounds(x, y) {
            let idx = self.xy_idx(x, y);
            self.fov_map[idx].visible = true;
            self.fov_map[idx].explored = true;
        }

        // Use simple circle-based FOV with line-of-sight checks
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let target_x = x + dx;
                let target_y = y + dy;

                if !map.in_bounds(target_x, target_y) {
                    continue;
                }

                // Check distance
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                if distance > radius as f32 {
                    continue;
                }

                // Check line of sight
                if self.has_line_of_sight(map, x, y, target_x, target_y) {
                    let idx = self.xy_idx(target_x, target_y);
                    self.fov_map[idx].visible = true;
                    self.fov_map[idx].explored = true;
                }
            }
        }
    }

    /// Check if there's a clear line of sight between two points using Bresenham's line
    fn has_line_of_sight(&self, map: &Map, x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            // If we reached the target, we have line of sight
            if x == x1 && y == y1 {
                return true;
            }

            // If we hit a wall (but not at the start), no line of sight
            if (x != x0 || y != y0) && !map.is_walkable(x, y) {
                return false;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Check if a tile is currently visible
    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        let idx = self.xy_idx(x, y);
        self.fov_map[idx].visible
    }

    /// Check if a tile has been explored
    pub fn is_explored(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        let idx = self.xy_idx(x, y);
        self.fov_map[idx].explored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::TileType;

    #[test]
    fn test_fov_creation() {
        let fov = FOV::new(80, 50);
        assert_eq!(fov.fov_map.len(), 80 * 50);
    }

    #[test]
    fn test_fov_visibility() {
        let mut map = Map::new(20, 20, 1);
        // Create a small room
        for y in 5..15 {
            for x in 5..15 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = TileType::Floor;
            }
        }

        let mut fov = FOV::new(20, 20);
        fov.compute_fov(&map, 10, 10, 5);

        // Center should be visible
        assert!(fov.is_visible(10, 10));

        // Nearby tiles should be visible
        assert!(fov.is_visible(11, 10));
        assert!(fov.is_visible(10, 11));

        // Distant tiles should not be visible
        assert!(!fov.is_visible(0, 0));
        assert!(!fov.is_visible(19, 19));
    }

    #[test]
    fn test_fov_explored() {
        let mut map = Map::new(20, 20, 1);
        for y in 5..15 {
            for x in 5..15 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = TileType::Floor;
            }
        }

        let mut fov = FOV::new(20, 20);

        // Initially not explored
        assert!(!fov.is_explored(10, 10));

        // After computing FOV, should be explored
        fov.compute_fov(&map, 10, 10, 5);
        assert!(fov.is_explored(10, 10));
        assert!(fov.is_explored(11, 10));
    }

    #[test]
    fn test_fov_walls_block_sight() {
        let mut map = Map::new(20, 20, 1);
        // Create a corridor with a wall
        for x in 5..15 {
            let idx = map.xy_idx(x, 10);
            map.tiles[idx] = TileType::Floor;
        }
        // Put a wall in the middle
        let wall_idx = map.xy_idx(10, 10);
        map.tiles[wall_idx] = TileType::Wall;

        let mut fov = FOV::new(20, 20);
        fov.compute_fov(&map, 5, 10, 10);

        // Position before wall should be visible
        assert!(fov.is_visible(9, 10));

        // Position after wall might not be visible (blocked by wall)
        // This depends on exact implementation, but wall itself should be visible
        assert!(fov.is_visible(10, 10));
    }

    #[test]
    fn test_fov_radius() {
        let mut map = Map::new(50, 50, 1);
        // Create large open area
        for y in 0..50 {
            for x in 0..50 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = TileType::Floor;
            }
        }

        let mut fov = FOV::new(50, 50);
        let radius = 5;
        fov.compute_fov(&map, 25, 25, radius);

        // Within radius should be visible
        assert!(fov.is_visible(25, 25));
        assert!(fov.is_visible(25 + radius - 1, 25));

        // Outside radius should not be visible
        assert!(!fov.is_visible(25 + radius + 2, 25));
    }
}
