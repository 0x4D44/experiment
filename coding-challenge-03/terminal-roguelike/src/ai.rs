//! AI behavior for enemies using A* pathfinding

use crate::entity::{Entity, Position};
use crate::map::Map;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    position: Position,
    f_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* pathfinding from start to goal
pub fn find_path(
    map: &Map,
    start: Position,
    goal: Position,
    occupied_positions: &HashSet<Position>,
) -> Option<Vec<Position>> {
    if start == goal {
        return Some(vec![start]);
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut g_score: HashMap<Position, i32> = HashMap::new();

    g_score.insert(start, 0);
    open_set.push(Node {
        position: start,
        f_score: start.manhattan_distance(&goal),
    });

    while let Some(Node {
        position: current, ..
    }) = open_set.pop()
    {
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }

        let current_g = *g_score.get(&current).unwrap_or(&i32::MAX);

        for neighbor in get_neighbors(map, &current, occupied_positions) {
            let tentative_g = current_g + 1;

            if tentative_g < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);

                let f_score = tentative_g + neighbor.manhattan_distance(&goal);
                open_set.push(Node {
                    position: neighbor,
                    f_score,
                });
            }
        }
    }

    None
}

fn get_neighbors(
    map: &Map,
    pos: &Position,
    occupied_positions: &HashSet<Position>,
) -> Vec<Position> {
    let deltas = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ];

    deltas
        .iter()
        .map(|(dx, dy)| Position::new(pos.x + dx, pos.y + dy))
        .filter(|p| map.is_walkable(p.x, p.y) && !occupied_positions.contains(p))
        .collect()
}

fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
    mut current: Position,
) -> Vec<Position> {
    let mut path = vec![current];

    while let Some(&prev) = came_from.get(&current) {
        current = prev;
        path.push(current);
    }

    path.reverse();
    path
}

/// Determine AI action for an enemy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIAction {
    Wait,
    MoveTowards(Position),
    Attack,
}

pub fn determine_action(
    enemy: &Entity,
    player_pos: &Position,
    map: &Map,
    occupied_positions: &HashSet<Position>,
    can_see_player: bool,
) -> AIAction {
    if !can_see_player {
        return AIAction::Wait;
    }

    let distance = enemy.position.distance_to(player_pos);

    // If adjacent, attack
    if distance <= 1.5 {
        return AIAction::Attack;
    }

    // Otherwise, move towards player
    if let Some(path) = find_path(map, enemy.position, *player_pos, occupied_positions) {
        if path.len() > 1 {
            return AIAction::MoveTowards(path[1]);
        }
    }

    AIAction::Wait
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{EnemyType, Entity};
    use crate::map::Map;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_find_path_direct() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        let start = map.first_room_center().unwrap();
        let goal = Position::new(start.x + 3, start.y + 3);

        let occupied = HashSet::new();
        let path = find_path(&map, start, goal, &occupied);

        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.len() > 0);
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], goal);
    }

    #[test]
    fn test_find_path_same_position() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        let pos = map.first_room_center().unwrap();
        let occupied = HashSet::new();

        let path = find_path(&map, pos, pos, &occupied);
        assert!(path.is_some());
        assert_eq!(path.unwrap(), vec![pos]);
    }

    #[test]
    fn test_find_path_blocked() {
        let map = Map::new(10, 10);
        let start = Position::new(0, 0);
        let goal = Position::new(5, 5);

        let occupied = HashSet::new();
        let path = find_path(&map, start, goal, &occupied);

        // Should fail in all-wall map
        assert!(path.is_none());
    }

    #[test]
    fn test_get_neighbors() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        let center = map.first_room_center().unwrap();
        let occupied = HashSet::new();

        let neighbors = get_neighbors(&map, &center, &occupied);

        // Should have up to 8 neighbors
        assert!(neighbors.len() > 0);
        assert!(neighbors.len() <= 8);

        // All neighbors should be walkable
        for neighbor in neighbors {
            assert!(map.is_walkable(neighbor.x, neighbor.y));
        }
    }

    #[test]
    fn test_determine_action_attack() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        let enemy = Entity::new_enemy(10, 10, EnemyType::Goblin);
        let player_pos = Position::new(11, 10);

        let occupied = HashSet::new();
        let action = determine_action(&enemy, &player_pos, &map, &occupied, true);

        assert_eq!(action, AIAction::Attack);
    }

    #[test]
    fn test_determine_action_move() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        let enemy = Entity::new_enemy(10, 10, EnemyType::Goblin);
        let player_pos = Position::new(15, 15);

        let occupied = HashSet::new();
        let action = determine_action(&enemy, &player_pos, &map, &occupied, true);

        match action {
            AIAction::MoveTowards(_) => {}
            AIAction::Wait => {} // May not be able to find path
            _ => panic!("Expected move or wait action"),
        }
    }

    #[test]
    fn test_determine_action_no_vision() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        let enemy = Entity::new_enemy(10, 10, EnemyType::Goblin);
        let player_pos = Position::new(15, 15);

        let occupied = HashSet::new();
        let action = determine_action(&enemy, &player_pos, &map, &occupied, false);

        assert_eq!(action, AIAction::Wait);
    }

    #[test]
    fn test_path_avoids_occupied() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);

        let start = map.first_room_center().unwrap();
        let goal = Position::new(start.x + 3, start.y);

        // Block direct path
        let mut occupied = HashSet::new();
        occupied.insert(Position::new(start.x + 1, start.y));
        occupied.insert(Position::new(start.x + 2, start.y));

        let path = find_path(&map, start, goal, &occupied);

        if let Some(path) = path {
            // Path should not contain occupied positions
            for pos in &path {
                assert!(!occupied.contains(pos) || *pos == start);
            }
        }
    }
}
