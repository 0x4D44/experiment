use crate::inventory::Item;
use rand::Rng;

/// Represents the player or an enemy
#[derive(Debug, Clone)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub symbol: char,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub xp: i32,
    pub level: i32,
    pub is_player: bool,
    pub ai_type: Option<AIType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIType {
    Zombie,      // Slow, high HP, low damage
    Goblin,      // Fast, low HP, medium damage
    Orc,         // Medium speed, high damage
    Demon,       // Fast, very high damage and HP
    Dragon,      // Boss - extremely powerful
}

impl Entity {
    /// Create the player entity
    pub fn new_player(x: i32, y: i32) -> Self {
        Entity {
            x,
            y,
            symbol: '@',
            name: "Hero".to_string(),
            hp: 30,
            max_hp: 30,
            attack: 5,
            defense: 2,
            xp: 0,
            level: 1,
            is_player: true,
            ai_type: None,
        }
    }

    /// Create an enemy based on dungeon depth
    pub fn new_enemy(x: i32, y: i32, depth: i32) -> Self {
        let mut rng = rand::thread_rng();

        // Different enemy types appear at different depths
        let ai_type = if depth >= 10 {
            // Boss level
            AIType::Dragon
        } else if depth >= 7 {
            match rng.gen_range(0..4) {
                0 => AIType::Zombie,
                1 => AIType::Goblin,
                2 => AIType::Orc,
                _ => AIType::Demon,
            }
        } else if depth >= 4 {
            match rng.gen_range(0..3) {
                0 => AIType::Zombie,
                1 => AIType::Goblin,
                _ => AIType::Orc,
            }
        } else {
            match rng.gen_range(0..2) {
                0 => AIType::Zombie,
                _ => AIType::Goblin,
            }
        };

        let (name, symbol, hp, attack, defense, xp_value) = match ai_type {
            AIType::Zombie => ("Zombie", 'Z', 16, 3, 0, 20),
            AIType::Goblin => ("Goblin", 'g', 8, 4, 1, 15),
            AIType::Orc => ("Orc", 'O', 12, 6, 2, 35),
            AIType::Demon => ("Demon", 'D', 20, 8, 3, 60),
            AIType::Dragon => ("Dragon Boss", 'X', 50, 12, 5, 200),
        };

        // Scale with depth
        let hp_bonus = (depth - 1) * 2;
        let attack_bonus = (depth - 1) / 2;

        Entity {
            x,
            y,
            symbol,
            name: name.to_string(),
            hp: hp + hp_bonus,
            max_hp: hp + hp_bonus,
            attack: attack + attack_bonus,
            defense,
            xp: xp_value,
            level: depth,
            is_player: false,
            ai_type: Some(ai_type),
        }
    }

    /// Check if entity is alive
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Take damage, returns true if entity died
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.hp -= damage.max(0);
        if self.hp < 0 {
            self.hp = 0;
        }
        !self.is_alive()
    }

    /// Heal the entity
    pub fn heal(&mut self, amount: i32) {
        self.hp += amount;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }

    /// Gain experience and potentially level up (handles multiple level-ups)
    pub fn gain_xp(&mut self, amount: i32) -> bool {
        self.xp += amount;
        let mut leveled_up = false;

        // Loop to handle multiple level-ups from a single XP gain
        while self.xp >= self.xp_to_next_level() {
            self.level_up();
            leveled_up = true;
        }

        leveled_up
    }

    /// Calculate XP needed for next level
    pub fn xp_to_next_level(&self) -> i32 {
        100 * self.level
    }

    /// Level up the entity
    fn level_up(&mut self) {
        self.level += 1;
        self.max_hp += 5;
        self.hp = self.max_hp;
        self.attack += 2;
        self.defense += 1;
    }

    /// Get movement speed (turns between moves)
    pub fn get_speed(&self) -> i32 {
        match self.ai_type {
            Some(AIType::Zombie) => 2,  // Moves every 2 turns
            Some(AIType::Goblin) => 1,  // Normal speed
            Some(AIType::Orc) => 1,
            Some(AIType::Demon) => 1,
            Some(AIType::Dragon) => 1,
            None => 1, // Player
        }
    }
}

/// Manages all entities in the game
pub struct EntityManager {
    pub player: Entity,
    pub enemies: Vec<Entity>,
    pub items: Vec<(i32, i32, Item)>, // Position and item
}

impl EntityManager {
    pub fn new(player: Entity) -> Self {
        EntityManager {
            player,
            enemies: Vec::new(),
            items: Vec::new(),
        }
    }

    /// Check if a position is blocked by an entity
    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        if self.player.x == x && self.player.y == y {
            return true;
        }
        self.enemies.iter().any(|e| e.x == x && e.y == y && e.is_alive())
    }

    /// Get enemy at position
    pub fn get_enemy_at(&mut self, x: i32, y: i32) -> Option<&mut Entity> {
        self.enemies.iter_mut().find(|e| e.x == x && e.y == y && e.is_alive())
    }

    /// Remove dead enemies
    pub fn remove_dead(&mut self) {
        self.enemies.retain(|e| e.is_alive());
    }

    /// Get item at position
    pub fn get_item_at(&self, x: i32, y: i32) -> Option<&Item> {
        self.items
            .iter()
            .find(|(ix, iy, _)| *ix == x && *iy == y)
            .map(|(_, _, item)| item)
    }

    /// Remove item at position
    pub fn remove_item_at(&mut self, x: i32, y: i32) -> Option<Item> {
        if let Some(idx) = self.items.iter().position(|(ix, iy, _)| *ix == x && *iy == y) {
            let (_, _, item) = self.items.remove(idx);
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Entity::new_player(10, 10);
        assert_eq!(player.x, 10);
        assert_eq!(player.y, 10);
        assert!(player.is_player);
        assert!(player.is_alive());
    }

    #[test]
    fn test_damage_and_death() {
        let mut entity = Entity::new_player(0, 0);
        let initial_hp = entity.hp;

        assert!(!entity.take_damage(5));
        assert_eq!(entity.hp, initial_hp - 5);
        assert!(entity.is_alive());

        assert!(entity.take_damage(100));
        assert!(!entity.is_alive());
        assert_eq!(entity.hp, 0);
    }

    #[test]
    fn test_healing() {
        let mut entity = Entity::new_player(0, 0);
        entity.take_damage(10);
        let damaged_hp = entity.hp;

        entity.heal(5);
        assert_eq!(entity.hp, damaged_hp + 5);

        entity.heal(100);
        assert_eq!(entity.hp, entity.max_hp);
    }

    #[test]
    fn test_xp_and_leveling() {
        let mut entity = Entity::new_player(0, 0);
        let initial_level = entity.level;
        let xp_needed = entity.xp_to_next_level();

        assert!(!entity.gain_xp(50));
        assert_eq!(entity.level, initial_level);

        assert!(entity.gain_xp(xp_needed));
        assert_eq!(entity.level, initial_level + 1);
    }

    #[test]
    fn test_multiple_level_ups() {
        let mut entity = Entity::new_player(0, 0);
        assert_eq!(entity.level, 1);
        assert_eq!(entity.xp, 0);

        // Test case from bug report: gaining 200 XP when needing 100 should level up twice
        // XP is cumulative (never reset):
        // Level 1: needs 100 XP cumulative to reach level 2
        // Level 2: needs 200 XP cumulative to reach level 3
        // So gaining 200 XP should level us up to level 3
        assert!(entity.gain_xp(200));
        assert_eq!(entity.level, 3);  // Fixed: should be level 3, not 2
        assert_eq!(entity.xp, 200);

        // Continue from level 3 - need 300 XP total to reach level 4
        // Already have 200, so need 100 more
        assert!(entity.gain_xp(100));
        assert_eq!(entity.level, 4);
        assert_eq!(entity.xp, 300);

        // Test gaining enough for multiple levels at once
        entity = Entity::new_player(0, 0);
        // Gaining 250 XP should get us to level 3 (needs 100 for L2, 200 for L3, 300 for L4)
        assert!(entity.gain_xp(250));
        assert_eq!(entity.level, 3);
        assert_eq!(entity.xp, 250);
    }

    #[test]
    fn test_enemy_creation() {
        let enemy = Entity::new_enemy(5, 5, 1);
        assert!(!enemy.is_player);
        assert!(enemy.ai_type.is_some());
        assert!(enemy.is_alive());
    }

    #[test]
    fn test_entity_manager() {
        let player = Entity::new_player(0, 0);
        let mut manager = EntityManager::new(player);

        assert!(!manager.is_blocked(5, 5));
        assert!(manager.is_blocked(0, 0)); // Player position

        let enemy = Entity::new_enemy(5, 5, 1);
        manager.enemies.push(enemy);
        assert!(manager.is_blocked(5, 5));
    }
}
