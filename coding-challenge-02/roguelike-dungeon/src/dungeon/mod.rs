use rand::Rng;

/// Represents a tile type in the dungeon
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    StairsDown,
    StairsUp,
}

/// A rectangular room in the dungeon
#[derive(Debug, Clone)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room { x, y, width, height }
    }

    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.x <= other.x + other.width
            && self.x + self.width >= other.x
            && self.y <= other.y + other.height
            && self.y + self.height >= other.y
    }
}

/// The dungeon map
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Room>,
    pub depth: i32,
}

impl Map {
    /// Create a new map filled with walls
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        let tiles = vec![TileType::Wall; (width * height) as usize];
        Map {
            width,
            height,
            tiles,
            rooms: Vec::new(),
            depth,
        }
    }

    /// Get the index of a tile from x, y coordinates
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// Check if coordinates are in bounds
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    /// Check if a tile is walkable
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        let idx = self.xy_idx(x, y);
        matches!(
            self.tiles[idx],
            TileType::Floor | TileType::StairsDown | TileType::StairsUp
        )
    }

    /// Generate a procedural dungeon using BSP rooms and corridors
    pub fn generate_dungeon(width: i32, height: i32, depth: i32) -> Self {
        let mut map = Map::new(width, height, depth);
        let mut rng = rand::thread_rng();

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 12;

        for _ in 0..MAX_ROOMS {
            let w = rng.gen_range(MIN_SIZE..=MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE..=MAX_SIZE);
            let x = rng.gen_range(1..width - w - 1);
            let y = rng.gen_range(1..height - h - 1);

            let new_room = Room::new(x, y, w, h);
            let mut ok = true;

            for other_room in &map.rooms {
                if new_room.intersects(other_room) {
                    ok = false;
                    break;
                }
            }

            if ok {
                map.create_room(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rng.gen_bool(0.5) {
                        map.create_h_tunnel(prev_x, new_x, prev_y);
                        map.create_v_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.create_v_tunnel(prev_y, new_y, prev_x);
                        map.create_h_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        // Add stairs (offset from center to avoid player spawn)
        if !map.rooms.is_empty() {
            let last_room = &map.rooms[map.rooms.len() - 1];
            let (cx, cy) = last_room.center();
            // Offset stairs by 1 tile from center
            let x = cx + 1;
            let y = cy;
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::StairsDown;
        }

        map
    }

    fn create_room(&mut self, room: &Room) {
        for y in room.y + 1..room.y + room.height {
            for x in room.x + 1..room.x + room.width {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in x1.min(x2)..=x1.max(x2) {
            let idx = self.xy_idx(x, y);
            if idx < self.tiles.len() {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in y1.min(y2)..=y1.max(y2) {
            let idx = self.xy_idx(x, y);
            if idx < self.tiles.len() {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_creation() {
        let room = Room::new(5, 5, 10, 8);
        assert_eq!(room.center(), (10, 9));
    }

    #[test]
    fn test_room_intersection() {
        let room1 = Room::new(0, 0, 10, 10);
        let room2 = Room::new(5, 5, 10, 10);
        let room3 = Room::new(20, 20, 10, 10);

        assert!(room1.intersects(&room2));
        assert!(!room1.intersects(&room3));
    }

    #[test]
    fn test_map_bounds() {
        let map = Map::new(80, 50, 1);
        assert!(map.in_bounds(0, 0));
        assert!(map.in_bounds(79, 49));
        assert!(!map.in_bounds(-1, 0));
        assert!(!map.in_bounds(80, 50));
    }

    #[test]
    fn test_dungeon_generation() {
        let map = Map::generate_dungeon(80, 50, 1);
        assert!(!map.rooms.is_empty());
        assert!(map.rooms.len() <= 30);

        // Check that at least some floor tiles exist
        let floor_count = map.tiles.iter().filter(|&&t| t == TileType::Floor).count();
        assert!(floor_count > 0);
    }

    #[test]
    fn test_xy_idx() {
        let map = Map::new(80, 50, 1);
        assert_eq!(map.xy_idx(0, 0), 0);
        assert_eq!(map.xy_idx(10, 5), 5 * 80 + 10);
    }
}
