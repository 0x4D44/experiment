//! Core data structures for game entities

use crate::items::Item;

/// Position in 2D space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// Combat and character statistics
#[derive(Debug, Clone)]
pub struct Stats {
    pub max_hp: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub xp: i32,
    pub level: i32,
}

impl Stats {
    pub fn new(max_hp: i32, attack: i32, defense: i32) -> Self {
        Self {
            max_hp,
            hp: max_hp,
            attack,
            defense,
            xp: 0,
            level: 1,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp = (self.hp - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn xp_to_next_level(&self) -> i32 {
        self.level * 100
    }

    pub fn add_xp(&mut self, amount: i32) -> bool {
        self.xp += amount;
        if self.xp >= self.xp_to_next_level() {
            self.level_up();
            return true;
        }
        false
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.xp = 0;
        self.max_hp += 10;
        self.hp = self.max_hp;
        self.attack += 2;
        self.defense += 1;
    }
}

/// Types of entities
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    Player,
    Enemy(EnemyType),
    #[allow(dead_code)]
    Item,
}

/// Different enemy types with varying difficulty
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Goblin,
    Orc,
    Troll,
    Dragon,
}

impl EnemyType {
    pub fn stats(&self) -> Stats {
        match self {
            EnemyType::Goblin => Stats::new(20, 5, 2),
            EnemyType::Orc => Stats::new(35, 8, 4),
            EnemyType::Troll => Stats::new(50, 12, 6),
            EnemyType::Dragon => Stats::new(100, 20, 10),
        }
    }

    pub fn xp_value(&self) -> i32 {
        match self {
            EnemyType::Goblin => 25,
            EnemyType::Orc => 50,
            EnemyType::Troll => 100,
            EnemyType::Dragon => 300,
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            EnemyType::Goblin => 'g',
            EnemyType::Orc => 'o',
            EnemyType::Troll => 'T',
            EnemyType::Dragon => 'D',
        }
    }

    pub fn name(&self) -> &str {
        match self {
            EnemyType::Goblin => "Goblin",
            EnemyType::Orc => "Orc",
            EnemyType::Troll => "Troll",
            EnemyType::Dragon => "Dragon",
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            EnemyType::Goblin => (0, 255, 0),
            EnemyType::Orc => (255, 165, 0),
            EnemyType::Troll => (139, 69, 19),
            EnemyType::Dragon => (255, 0, 0),
        }
    }
}

/// Main entity structure
#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Position,
    pub entity_type: EntityType,
    pub stats: Option<Stats>,
    pub inventory: Vec<Item>,
    pub equipped_weapon: Option<Item>,
    pub equipped_armor: Option<Item>,
    #[allow(dead_code)]
    pub blocks_movement: bool,
}

impl Entity {
    pub fn new_player(x: i32, y: i32) -> Self {
        Self {
            position: Position::new(x, y),
            entity_type: EntityType::Player,
            stats: Some(Stats::new(100, 10, 5)),
            inventory: Vec::new(),
            equipped_weapon: None,
            equipped_armor: None,
            blocks_movement: true,
        }
    }

    pub fn new_enemy(x: i32, y: i32, enemy_type: EnemyType) -> Self {
        Self {
            position: Position::new(x, y),
            entity_type: EntityType::Enemy(enemy_type),
            stats: Some(enemy_type.stats()),
            inventory: Vec::new(),
            equipped_weapon: None,
            equipped_armor: None,
            blocks_movement: true,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.stats.as_ref().is_some_and(|s| s.is_alive())
    }

    pub fn symbol(&self) -> char {
        match self.entity_type {
            EntityType::Player => '@',
            EntityType::Enemy(enemy_type) => enemy_type.symbol(),
            EntityType::Item => '?',
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self.entity_type {
            EntityType::Player => (255, 255, 255),
            EntityType::Enemy(enemy_type) => enemy_type.color(),
            EntityType::Item => (255, 255, 0),
        }
    }

    pub fn name(&self) -> String {
        match self.entity_type {
            EntityType::Player => "You".to_string(),
            EntityType::Enemy(enemy_type) => enemy_type.name().to_string(),
            EntityType::Item => "Item".to_string(),
        }
    }

    pub fn total_attack(&self) -> i32 {
        let base = self.stats.as_ref().map_or(0, |s| s.attack);
        let weapon_bonus = self
            .equipped_weapon
            .as_ref()
            .map_or(0, |w| w.attack_bonus());
        base + weapon_bonus
    }

    pub fn total_defense(&self) -> i32 {
        let base = self.stats.as_ref().map_or(0, |s| s.defense);
        let armor_bonus = self
            .equipped_armor
            .as_ref()
            .map_or(0, |a| a.defense_bonus());
        base + armor_bonus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_distance() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(3, 4);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_position_manhattan_distance() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(3, 4);
        assert_eq!(p1.manhattan_distance(&p2), 7);
    }

    #[test]
    fn test_stats_damage() {
        let mut stats = Stats::new(100, 10, 5);
        stats.take_damage(30);
        assert_eq!(stats.hp, 70);
        stats.take_damage(100);
        assert_eq!(stats.hp, 0);
        assert!(!stats.is_alive());
    }

    #[test]
    fn test_stats_healing() {
        let mut stats = Stats::new(100, 10, 5);
        stats.take_damage(50);
        stats.heal(20);
        assert_eq!(stats.hp, 70);
        stats.heal(100);
        assert_eq!(stats.hp, 100);
    }

    #[test]
    fn test_stats_level_up() {
        let mut stats = Stats::new(100, 10, 5);
        let leveled = stats.add_xp(100);
        assert!(leveled);
        assert_eq!(stats.level, 2);
        assert_eq!(stats.max_hp, 110);
        assert_eq!(stats.attack, 12);
        assert_eq!(stats.defense, 6);
    }

    #[test]
    fn test_enemy_types() {
        let goblin_stats = EnemyType::Goblin.stats();
        assert_eq!(goblin_stats.max_hp, 20);
        assert_eq!(EnemyType::Goblin.xp_value(), 25);

        let dragon_stats = EnemyType::Dragon.stats();
        assert!(dragon_stats.max_hp > goblin_stats.max_hp);
    }

    #[test]
    fn test_entity_creation() {
        let player = Entity::new_player(5, 5);
        assert_eq!(player.position.x, 5);
        assert_eq!(player.position.y, 5);
        assert!(player.is_alive());

        let goblin = Entity::new_enemy(10, 10, EnemyType::Goblin);
        assert!(goblin.is_alive());
        assert_eq!(goblin.symbol(), 'g');
    }
}
