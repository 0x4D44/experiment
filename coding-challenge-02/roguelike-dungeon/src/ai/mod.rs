use crate::dungeon::Map;
use crate::entity::{Entity, EntityManager, AIType};
use crate::fov::FOV;

/// AI system for enemy behavior
pub struct AISystem;

impl AISystem {
    /// Update all enemy AI
    pub fn update_enemies(
        entities: &mut EntityManager,
        map: &Map,
        fov: &FOV,
        turn_count: u32,
    ) -> Vec<(usize, i32, i32)> {
        let player_pos = (entities.player.x, entities.player.y);
        let mut moves = Vec::new();

        for (idx, enemy) in entities.enemies.iter().enumerate() {
            if !enemy.is_alive() {
                continue;
            }

            // Check if it's this enemy's turn based on speed
            let speed = enemy.get_speed();
            if turn_count % speed as u32 != 0 {
                continue;
            }

            // Check if enemy can see player
            if !fov.is_visible(enemy.x, enemy.y) {
                continue;
            }

            let action = Self::decide_action(enemy, player_pos, map, entities);
            if let Some((dx, dy)) = action {
                moves.push((idx, dx, dy));
            }
        }

        moves
    }

    /// Decide what action an enemy should take
    fn decide_action(
        enemy: &Entity,
        player_pos: (i32, i32),
        map: &Map,
        entities: &EntityManager,
    ) -> Option<(i32, i32)> {
        let (px, py) = player_pos;
        let (ex, ey) = (enemy.x, enemy.y);

        // Calculate distance to player
        let dx = px - ex;
        let dy = py - ey;
        let distance = ((dx * dx + dy * dy) as f32).sqrt();

        // Different behavior based on AI type
        match enemy.ai_type {
            Some(AIType::Zombie) => {
                // Zombies: slow, direct approach
                if distance <= 1.5 {
                    // Adjacent, attack
                    return Some((dx, dy));
                }
                Self::move_towards(ex, ey, px, py, map, entities)
            }
            Some(AIType::Goblin) => {
                // Goblins: fast, aggressive
                if distance <= 1.5 {
                    return Some((dx, dy));
                }
                Self::move_towards(ex, ey, px, py, map, entities)
            }
            Some(AIType::Orc) => {
                // Orcs: charge when close
                if distance <= 1.5 {
                    return Some((dx, dy));
                } else if distance < 5.0 {
                    Self::move_towards(ex, ey, px, py, map, entities)
                } else {
                    None
                }
            }
            Some(AIType::Demon) => {
                // Demons: smart pathfinding
                if distance <= 1.5 {
                    return Some((dx, dy));
                }
                Self::move_towards(ex, ey, px, py, map, entities)
            }
            Some(AIType::Dragon) => {
                // Dragon boss: always aggressive
                if distance <= 1.5 {
                    return Some((dx, dy));
                }
                Self::move_towards(ex, ey, px, py, map, entities)
            }
            None => None,
        }
    }

    /// Calculate a move towards a target
    fn move_towards(
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
        map: &Map,
        entities: &EntityManager,
    ) -> Option<(i32, i32)> {
        let dx = to_x - from_x;
        let dy = to_y - from_y;

        // Normalize movement to one tile
        let step_x = dx.signum();
        let step_y = dy.signum();

        // Try to move diagonally first
        if step_x != 0 && step_y != 0 {
            let new_x = from_x + step_x;
            let new_y = from_y + step_y;
            if map.is_walkable(new_x, new_y) && !entities.is_blocked(new_x, new_y) {
                return Some((step_x, step_y));
            }
        }

        // Try horizontal movement
        if step_x != 0 {
            let new_x = from_x + step_x;
            if map.is_walkable(new_x, from_y) && !entities.is_blocked(new_x, from_y) {
                return Some((step_x, 0));
            }
        }

        // Try vertical movement
        if step_y != 0 {
            let new_y = from_y + step_y;
            if map.is_walkable(from_x, new_y) && !entities.is_blocked(from_x, new_y) {
                return Some((0, step_y));
            }
        }

        None
    }

    /// Check if position is adjacent to player
    pub fn is_adjacent_to_player(enemy_x: i32, enemy_y: i32, player_x: i32, player_y: i32) -> bool {
        let dx = (enemy_x - player_x).abs();
        let dy = (enemy_y - player_y).abs();
        dx <= 1 && dy <= 1 && (dx + dy) > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_check() {
        assert!(AISystem::is_adjacent_to_player(5, 5, 5, 6));
        assert!(AISystem::is_adjacent_to_player(5, 5, 6, 5));
        assert!(AISystem::is_adjacent_to_player(5, 5, 6, 6));
        assert!(!AISystem::is_adjacent_to_player(5, 5, 5, 5)); // Same position
        assert!(!AISystem::is_adjacent_to_player(5, 5, 7, 5)); // Too far
    }

    #[test]
    fn test_move_towards() {
        let mut map = Map::new(20, 20, 1);
        // Create open area
        for y in 0..20 {
            for x in 0..20 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = crate::dungeon::TileType::Floor;
            }
        }

        let player = Entity::new_player(10, 10);
        let entities = EntityManager::new(player);

        // Test moving towards player
        let action = AISystem::move_towards(5, 5, 10, 10, &map, &entities);
        assert!(action.is_some());

        if let Some((dx, dy)) = action {
            // Should move towards player
            assert!(dx != 0 || dy != 0);
        }
    }

    #[test]
    fn test_enemy_speed() {
        let zombie = Entity::new_enemy(0, 0, 1);
        let goblin = Entity::new_enemy(0, 0, 1);

        // Check speed (this is a simple check, actual values depend on AI type)
        assert!(zombie.get_speed() >= 1);
        assert!(goblin.get_speed() >= 1);
    }
}
