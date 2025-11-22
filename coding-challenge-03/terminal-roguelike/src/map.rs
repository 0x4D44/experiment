//! Dungeon map generation and management

use crate::entity::Position;
use rand::Rng;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    StairsDown,
}

impl Tile {
    pub fn blocks_movement(&self) -> bool {
        matches!(self, Tile::Wall)
    }

    pub fn blocks_sight(&self) -> bool {
        matches!(self, Tile::Wall)
    }

    pub fn symbol(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::StairsDown => '>',
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Tile::Wall => (100, 100, 100),
            Tile::Floor => (50, 50, 50),
            Tile::StairsDown => (255, 255, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center(&self) -> Position {
        Position::new(self.x + self.width / 2, self.y + self.height / 2)
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.x <= other.x + other.width
            && self.x + self.width >= other.x
            && self.y <= other.y + other.height
            && self.y + self.height >= other.y
    }

    #[allow(dead_code)]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
    pub revealed: Vec<Vec<bool>>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let tiles = vec![vec![Tile::Wall; width as usize]; height as usize];
        let revealed = vec![vec![false; width as usize]; height as usize];

        Self {
            width,
            height,
            tiles,
            rooms: Vec::new(),
            revealed,
        }
    }

    pub fn generate(_level: i32, rng: &mut impl Rng) -> Self {
        let mut map = Self::new(MAP_WIDTH, MAP_HEIGHT);

        let max_rooms = 30;
        let min_size = 6;
        let max_size = 12;

        for _ in 0..max_rooms {
            let width = rng.gen_range(min_size..=max_size);
            let height = rng.gen_range(min_size..=max_size);
            let x = rng.gen_range(1..MAP_WIDTH - width - 1);
            let y = rng.gen_range(1..MAP_HEIGHT - height - 1);

            let new_room = Room::new(x, y, width, height);

            let intersects = map.rooms.iter().any(|room| new_room.intersects(room));

            if !intersects {
                map.create_room(&new_room);

                if !map.rooms.is_empty() {
                    let prev_center = map.rooms.last().unwrap().center();
                    let new_center = new_room.center();

                    if rng.gen_bool(0.5) {
                        map.create_horizontal_tunnel(prev_center.x, new_center.x, prev_center.y);
                        map.create_vertical_tunnel(prev_center.y, new_center.y, new_center.x);
                    } else {
                        map.create_vertical_tunnel(prev_center.y, new_center.y, prev_center.x);
                        map.create_horizontal_tunnel(prev_center.x, new_center.x, new_center.y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        // Place stairs in the last room
        if let Some(last_room) = map.rooms.last() {
            let center = last_room.center();
            map.tiles[center.y as usize][center.x as usize] = Tile::StairsDown;
        }

        map
    }

    fn create_room(&mut self, room: &Room) {
        for y in room.y + 1..room.y + room.height {
            for x in room.x + 1..room.x + room.width {
                if x > 0 && x < self.width && y > 0 && y < self.height {
                    self.tiles[y as usize][x as usize] = Tile::Floor;
                }
            }
        }
    }

    fn create_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        let start_x = x1.min(x2);
        let end_x = x1.max(x2);

        for x in start_x..=end_x {
            if x > 0 && x < self.width && y > 0 && y < self.height {
                self.tiles[y as usize][x as usize] = Tile::Floor;
            }
        }
    }

    fn create_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        let start_y = y1.min(y2);
        let end_y = y1.max(y2);

        for y in start_y..=end_y {
            if x > 0 && x < self.width && y > 0 && y < self.height {
                self.tiles[y as usize][x as usize] = Tile::Floor;
            }
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        !self.tiles[y as usize][x as usize].blocks_movement()
    }

    pub fn blocks_sight(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return true;
        }
        self.tiles[y as usize][x as usize].blocks_sight()
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(self.tiles[y as usize][x as usize])
    }

    pub fn reveal(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.revealed[y as usize][x as usize] = true;
        }
    }

    pub fn is_revealed(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        self.revealed[y as usize][x as usize]
    }

    pub fn first_room_center(&self) -> Option<Position> {
        self.rooms.first().map(|room| room.center())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_tile_properties() {
        assert!(Tile::Wall.blocks_movement());
        assert!(Tile::Wall.blocks_sight());
        assert!(!Tile::Floor.blocks_movement());
        assert!(!Tile::Floor.blocks_sight());
    }

    #[test]
    fn test_room_creation() {
        let room = Room::new(5, 5, 10, 8);
        assert_eq!(room.x, 5);
        assert_eq!(room.width, 10);

        let center = room.center();
        assert_eq!(center.x, 10);
        assert_eq!(center.y, 9);
    }

    #[test]
    fn test_room_intersection() {
        let room1 = Room::new(5, 5, 10, 10);
        let room2 = Room::new(10, 10, 10, 10);
        let room3 = Room::new(20, 20, 10, 10);

        assert!(room1.intersects(&room2));
        assert!(!room1.intersects(&room3));
    }

    #[test]
    fn test_room_contains() {
        let room = Room::new(5, 5, 10, 10);
        assert!(room.contains(7, 7));
        assert!(room.contains(5, 5));
        assert!(!room.contains(15, 15));
        assert!(!room.contains(4, 4));
    }

    #[test]
    fn test_map_creation() {
        let map = Map::new(80, 45);
        assert_eq!(map.width, 80);
        assert_eq!(map.height, 45);
        assert_eq!(map.tiles.len(), 45);
        assert_eq!(map.tiles[0].len(), 80);
    }

    #[test]
    fn test_map_generation() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        assert!(!map.rooms.is_empty());
        assert!(map.rooms.len() <= 30);

        // Check that first room has walkable floor
        let first_room = &map.rooms[0];
        let center = first_room.center();
        assert!(map.is_walkable(center.x, center.y));
    }

    #[test]
    fn test_map_walkability() {
        let map = Map::new(10, 10);
        assert!(!map.is_walkable(-1, 0));
        assert!(!map.is_walkable(0, -1));
        assert!(!map.is_walkable(10, 0));
        assert!(!map.is_walkable(0, 10));
    }

    #[test]
    fn test_map_reveal() {
        let mut map = Map::new(10, 10);
        assert!(!map.is_revealed(5, 5));

        map.reveal(5, 5);
        assert!(map.is_revealed(5, 5));
    }

    #[test]
    fn test_stairs_placement() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        // Check that stairs exist somewhere
        let has_stairs = map
            .tiles
            .iter()
            .any(|row| row.iter().any(|tile| *tile == Tile::StairsDown));
        assert!(has_stairs);
    }
}
