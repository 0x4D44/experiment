use crate::hex::HexCoord;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Red,   // Connects North-South
    Blue,  // Connects East-West
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    size: i32,
    board: HashMap<HexCoord, Player>,
    current_player: Player,
    winner: Option<Player>,
    move_count: usize,
}

impl Game {
    pub fn new(size: i32) -> Self {
        Game {
            size,
            board: HashMap::new(),
            current_player: Player::Red,
            winner: None,
            move_count: 0,
        }
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn get(&self, pos: HexCoord) -> Option<Player> {
        self.board.get(&pos).copied()
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn current_player_is_human(&self) -> bool {
        self.current_player == Player::Red
    }

    pub fn is_valid_move(&self, pos: HexCoord) -> bool {
        pos.is_valid(self.size) && !self.board.contains_key(&pos)
    }

    pub fn make_move(&mut self, pos: HexCoord) -> bool {
        if !self.is_valid_move(pos) {
            return false;
        }

        self.board.insert(pos, self.current_player);
        self.move_count += 1;

        if self.check_winner(self.current_player) {
            self.winner = Some(self.current_player);
        } else {
            self.current_player = self.current_player.other();
        }

        true
    }

    pub fn is_game_over(&self) -> bool {
        self.winner.is_some()
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn move_count(&self) -> usize {
        self.move_count
    }

    pub fn empty_positions(&self) -> Vec<HexCoord> {
        let mut positions = Vec::new();
        for q in 0..self.size {
            for r in 0..self.size {
                let pos = HexCoord::new(q, r);
                if !self.board.contains_key(&pos) {
                    positions.push(pos);
                }
            }
        }
        positions
    }

    fn check_winner(&self, player: Player) -> bool {
        // Use BFS to check if player has connected their sides
        match player {
            Player::Red => self.is_connected_north_south(player),
            Player::Blue => self.is_connected_east_west(player),
        }
    }

    fn is_connected_north_south(&self, player: Player) -> bool {
        // Find all player pieces on north edge
        let mut start_positions = Vec::new();
        for q in 0..self.size {
            let pos = HexCoord::new(q, 0);
            if self.board.get(&pos) == Some(&player) {
                start_positions.push(pos);
            }
        }

        // BFS from each north edge position to see if we can reach south edge
        for start in start_positions {
            if self.path_exists_to_south(start, player) {
                return true;
            }
        }

        false
    }

    fn is_connected_east_west(&self, player: Player) -> bool {
        // Find all player pieces on west edge
        let mut start_positions = Vec::new();
        for r in 0..self.size {
            let pos = HexCoord::new(0, r);
            if self.board.get(&pos) == Some(&player) {
                start_positions.push(pos);
            }
        }

        // BFS from each west edge position to see if we can reach east edge
        for start in start_positions {
            if self.path_exists_to_east(start, player) {
                return true;
            }
        }

        false
    }

    fn path_exists_to_south(&self, start: HexCoord, player: Player) -> bool {
        use std::collections::{HashSet, VecDeque};

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current.is_south_edge(self.size) {
                return true;
            }

            for neighbor in current.neighbors() {
                if !visited.contains(&neighbor)
                    && neighbor.is_valid(self.size)
                    && self.board.get(&neighbor) == Some(&player)
                {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        false
    }

    fn path_exists_to_east(&self, start: HexCoord, player: Player) -> bool {
        use std::collections::{HashSet, VecDeque};

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current.is_east_edge(self.size) {
                return true;
            }

            for neighbor in current.neighbors() {
                if !visited.contains(&neighbor)
                    && neighbor.is_valid(self.size)
                    && self.board.get(&neighbor) == Some(&player)
                {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new(11);
        assert_eq!(game.size(), 11);
        assert_eq!(game.current_player(), Player::Red);
        assert!(!game.is_game_over());
        assert_eq!(game.move_count(), 0);
    }

    #[test]
    fn test_valid_move() {
        let mut game = Game::new(11);
        let pos = HexCoord::new(5, 5);

        assert!(game.is_valid_move(pos));
        assert!(game.make_move(pos));
        assert_eq!(game.get(pos), Some(Player::Red));
        assert!(!game.is_valid_move(pos)); // Can't play on occupied cell
    }

    #[test]
    fn test_invalid_move() {
        let mut game = Game::new(11);
        let pos = HexCoord::new(11, 5); // Out of bounds

        assert!(!game.is_valid_move(pos));
        assert!(!game.make_move(pos));
    }

    #[test]
    fn test_player_alternation() {
        let mut game = Game::new(11);

        assert_eq!(game.current_player(), Player::Red);
        game.make_move(HexCoord::new(0, 0));

        assert_eq!(game.current_player(), Player::Blue);
        game.make_move(HexCoord::new(1, 1));

        assert_eq!(game.current_player(), Player::Red);
    }

    #[test]
    fn test_red_wins_north_south() {
        let mut game = Game::new(5);

        // Create a vertical path for Red
        for r in 0..5 {
            game.make_move(HexCoord::new(2, r)); // Red
            if r < 4 {
                game.make_move(HexCoord::new(3, r)); // Blue (blocking attempts)
            }
        }

        assert!(game.is_game_over());
        assert_eq!(game.get_winner(), Some(Player::Red));
    }

    #[test]
    fn test_blue_wins_east_west() {
        let mut game = Game::new(5);

        // Create a horizontal path for Blue
        game.make_move(HexCoord::new(0, 0)); // Red
        for q in 0..5 {
            game.make_move(HexCoord::new(q, 2)); // Blue
            if q < 4 {
                game.make_move(HexCoord::new(q, 3)); // Red (blocking attempts)
            }
        }

        assert!(game.is_game_over());
        assert_eq!(game.get_winner(), Some(Player::Blue));
    }

    #[test]
    fn test_empty_positions() {
        let mut game = Game::new(3);
        assert_eq!(game.empty_positions().len(), 9);

        game.make_move(HexCoord::new(0, 0));
        assert_eq!(game.empty_positions().len(), 8);

        game.make_move(HexCoord::new(1, 1));
        assert_eq!(game.empty_positions().len(), 7);
    }
}
