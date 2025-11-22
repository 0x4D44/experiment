//! Main game state and logic

use crate::ai::{determine_action, AIAction};
use crate::combat;
use crate::entity::{EnemyType, Entity, EntityType, Position};
use crate::fov;
use crate::highscore::{HighScore, HighScores};
use crate::items::{generate_random_item, Item, ItemType};
use crate::map::{Map, Tile};
use rand::Rng;
use std::collections::HashSet;

const MAX_INVENTORY_SIZE: usize = 10;

pub struct Game {
    pub map: Map,
    pub player: Entity,
    pub enemies: Vec<Entity>,
    pub items: Vec<(Position, Item)>,
    pub level: i32,
    pub messages: Vec<String>,
    pub visible_tiles: HashSet<Position>,
    pub game_over: bool,
    pub high_scores: HighScores,
    rng: rand::rngs::ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let map = Map::generate(1, &mut rng);
        let player_pos = map.first_room_center().unwrap();

        let mut game = Self {
            map,
            player: Entity::new_player(player_pos.x, player_pos.y),
            enemies: Vec::new(),
            items: Vec::new(),
            level: 1,
            messages: vec!["Welcome to the dungeon! Press '?' for help.".to_string()],
            visible_tiles: HashSet::new(),
            game_over: false,
            high_scores: HighScores::load(),
            rng,
        };

        game.spawn_enemies();
        game.spawn_items();
        game.update_fov();

        game
    }

    fn spawn_enemies(&mut self) {
        let num_enemies = 3 + self.level * 2;
        let num_rooms = self.map.rooms.len();

        for _ in 0..num_enemies {
            if num_rooms == 0 {
                break;
            }

            let room_idx = self.rng.gen_range(0..num_rooms);
            let room = &self.map.rooms[room_idx];
            let x = self.rng.gen_range(room.x + 1..room.x + room.width - 1);
            let y = self.rng.gen_range(room.y + 1..room.y + room.height - 1);

            // Don't spawn on player
            if x == self.player.position.x && y == self.player.position.y {
                continue;
            }

            let enemy_type = self.random_enemy_type();
            self.enemies.push(Entity::new_enemy(x, y, enemy_type));
        }
    }

    fn spawn_items(&mut self) {
        let num_items = 2 + self.level;
        let num_rooms = self.map.rooms.len();

        for _ in 0..num_items {
            if num_rooms == 0 {
                break;
            }

            let room_idx = self.rng.gen_range(0..num_rooms);
            let room = &self.map.rooms[room_idx];
            let x = self.rng.gen_range(room.x + 1..room.x + room.width - 1);
            let y = self.rng.gen_range(room.y + 1..room.y + room.height - 1);

            let item = generate_random_item(self.level, &mut self.rng);
            self.items.push((Position::new(x, y), item));
        }
    }

    fn random_enemy_type(&mut self) -> EnemyType {
        let roll = self.rng.gen_range(0..100);
        match self.level {
            1..=2 => {
                if roll < 80 {
                    EnemyType::Goblin
                } else {
                    EnemyType::Orc
                }
            }
            3..=5 => {
                if roll < 40 {
                    EnemyType::Goblin
                } else if roll < 85 {
                    EnemyType::Orc
                } else {
                    EnemyType::Troll
                }
            }
            _ => {
                if roll < 20 {
                    EnemyType::Goblin
                } else if roll < 50 {
                    EnemyType::Orc
                } else if roll < 90 {
                    EnemyType::Troll
                } else {
                    EnemyType::Dragon
                }
            }
        }
    }

    pub fn try_move_player(&mut self, dx: i32, dy: i32) {
        let new_x = self.player.position.x + dx;
        let new_y = self.player.position.y + dy;

        // Check for enemy at target position
        if let Some(enemy_idx) = self
            .enemies
            .iter()
            .position(|e| e.position.x == new_x && e.position.y == new_y && e.is_alive())
        {
            // Attack enemy
            if let Some(result) =
                combat::attack(&self.player, &mut self.enemies[enemy_idx], &mut self.rng)
            {
                self.add_message(format!(
                    "You attack {} for {} damage!",
                    result.defender_name, result.damage
                ));

                if result.defender_killed {
                    let enemy_type = match self.enemies[enemy_idx].entity_type {
                        EntityType::Enemy(et) => et,
                        _ => EnemyType::Goblin,
                    };
                    let xp = enemy_type.xp_value();
                    self.add_message(format!("You killed {}! (+{} XP)", result.defender_name, xp));

                    let leveled_up = if let Some(stats) = self.player.stats.as_mut() {
                        stats.add_xp(xp)
                    } else {
                        false
                    };

                    if leveled_up {
                        let new_level = self.player.stats.as_ref().unwrap().level;
                        self.add_message(format!("Level up! You are now level {}!", new_level));
                    }
                }
            }
        } else if self.map.is_walkable(new_x, new_y) {
            // Move player
            self.player.position.x = new_x;
            self.player.position.y = new_y;
            self.update_fov();
        }
    }

    pub fn process_enemy_turns(&mut self) {
        let player_pos = self.player.position;

        let mut occupied: HashSet<Position> = HashSet::new();
        occupied.insert(self.player.position);
        for enemy in &self.enemies {
            if enemy.is_alive() {
                occupied.insert(enemy.position);
            }
        }

        for i in 0..self.enemies.len() {
            if !self.enemies[i].is_alive() {
                continue;
            }

            let can_see = self.visible_tiles.contains(&self.enemies[i].position);
            let mut temp_occupied = occupied.clone();
            temp_occupied.remove(&self.enemies[i].position);

            let action = determine_action(
                &self.enemies[i],
                &player_pos,
                &self.map,
                &temp_occupied,
                can_see,
            );

            match action {
                AIAction::Attack => {
                    if let Some(result) =
                        combat::attack(&self.enemies[i], &mut self.player, &mut self.rng)
                    {
                        self.add_message(format!(
                            "{} attacks you for {} damage!",
                            result.attacker_name, result.damage
                        ));

                        if result.defender_killed {
                            self.game_over = true;
                            self.handle_death();
                        }
                    }
                }
                AIAction::MoveTowards(new_pos) => {
                    occupied.remove(&self.enemies[i].position);
                    self.enemies[i].position = new_pos;
                    occupied.insert(new_pos);
                }
                AIAction::Wait => {}
            }
        }
    }

    fn handle_death(&mut self) {
        self.add_message("You have died! Press 'n' for new game or 'q' to quit.".to_string());

        let score = self.calculate_score();
        if self.high_scores.is_high_score(score) {
            self.high_scores
                .add_score(HighScore::new(score, self.level));
            let _ = self.high_scores.save();
        }
    }

    fn calculate_score(&self) -> i32 {
        let base_score = self.level * 100;
        let xp_score = self
            .player
            .stats
            .as_ref()
            .map_or(0, |s| s.xp + (s.level - 1) * 100);
        base_score + xp_score
    }

    pub fn pickup_item(&mut self) {
        let player_pos = self.player.position;

        if let Some(item_idx) = self.items.iter().position(|(pos, _)| *pos == player_pos) {
            let (_, item) = self.items.remove(item_idx);

            if self.player.inventory.len() >= MAX_INVENTORY_SIZE {
                self.add_message("Inventory full!".to_string());
                self.items.push((player_pos, item));
                return;
            }

            self.add_message(format!("Picked up {}!", item.name));
            self.player.inventory.push(item);
        } else {
            self.add_message("Nothing to pick up here.".to_string());
        }
    }

    pub fn use_item(&mut self, index: usize) {
        if index >= self.player.inventory.len() {
            return;
        }

        let item = self.player.inventory[index].clone();

        match item.item_type {
            ItemType::Potion => {
                if let Some(stats) = self.player.stats.as_mut() {
                    stats.heal(item.heal_amount);
                    self.add_message(format!(
                        "Used {}! Healed {} HP.",
                        item.name, item.heal_amount
                    ));
                    self.player.inventory.remove(index);
                }
            }
            ItemType::Weapon => {
                if let Some(old_weapon) = self.player.equipped_weapon.take() {
                    self.player.inventory.push(old_weapon);
                }
                self.player.equipped_weapon = Some(item);
                self.player.inventory.remove(index);
                self.add_message(format!(
                    "Equipped {}!",
                    self.player.equipped_weapon.as_ref().unwrap().name
                ));
            }
            ItemType::Armor => {
                if let Some(old_armor) = self.player.equipped_armor.take() {
                    self.player.inventory.push(old_armor);
                }
                self.player.equipped_armor = Some(item);
                self.player.inventory.remove(index);
                self.add_message(format!(
                    "Equipped {}!",
                    self.player.equipped_armor.as_ref().unwrap().name
                ));
            }
        }
    }

    pub fn drop_item(&mut self, index: usize) {
        if index >= self.player.inventory.len() {
            return;
        }

        let item = self.player.inventory.remove(index);
        self.add_message(format!("Dropped {}.", item.name));
        self.items.push((self.player.position, item));
    }

    pub fn descend_stairs(&mut self) {
        let player_pos = self.player.position;
        if let Some(tile) = self.map.get_tile(player_pos.x, player_pos.y) {
            if tile == Tile::StairsDown {
                self.level += 1;
                self.add_message(format!("You descend to level {}!", self.level));

                // Generate new level
                self.map = Map::generate(self.level, &mut self.rng);
                let start_pos = self.map.first_room_center().unwrap();
                self.player.position = start_pos;

                self.enemies.clear();
                self.items.clear();
                self.spawn_enemies();
                self.spawn_items();
                self.update_fov();
            } else {
                self.add_message("No stairs here.".to_string());
            }
        }
    }

    fn update_fov(&mut self) {
        self.visible_tiles =
            fov::calculate_fov(&self.map, &self.player.position, fov::default_fov_radius());

        for pos in &self.visible_tiles {
            self.map.reveal(pos.x, pos.y);
        }
    }

    fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
        if self.messages.len() > 50 {
            self.messages.remove(0);
        }
    }

    pub fn is_player_turn(&self) -> bool {
        !self.game_over
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    #[allow(dead_code)]
    pub fn get_messages(&self) -> &[String] {
        &self.messages
    }

    #[allow(dead_code)]
    pub fn get_visible_enemies(&self) -> Vec<&Entity> {
        self.enemies
            .iter()
            .filter(|e| e.is_alive() && self.visible_tiles.contains(&e.position))
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_visible_items(&self) -> Vec<&(Position, Item)> {
        self.items
            .iter()
            .filter(|(pos, _)| self.visible_tiles.contains(pos))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new();
        assert_eq!(game.level, 1);
        assert!(game.player.is_alive());
        assert!(!game.enemies.is_empty());
        assert!(!game.game_over);
    }

    #[test]
    fn test_player_movement() {
        let mut game = Game::new();
        let _initial_pos = game.player.position;

        // Try moving (may not succeed if blocked)
        game.try_move_player(1, 0);

        // Position should change or stay same (not corrupt)
        assert!(game.player.position.x >= 0);
        assert!(game.player.position.y >= 0);
    }

    #[test]
    fn test_item_pickup() {
        let mut game = Game::new();

        // Place item at player position
        let item = Item::new_potion("Test Potion", 20);
        game.items.push((game.player.position, item));

        let initial_inventory = game.player.inventory.len();
        game.pickup_item();

        assert_eq!(game.player.inventory.len(), initial_inventory + 1);
    }

    #[test]
    fn test_item_use() {
        let mut game = Game::new();

        // Damage player
        let damaged_hp = if let Some(stats) = game.player.stats.as_mut() {
            stats.take_damage(30);
            stats.hp
        } else {
            0
        };

        // Add and use potion
        game.player.inventory.push(Item::new_potion("Potion", 20));
        game.use_item(0);

        if let Some(stats) = game.player.stats.as_ref() {
            assert!(stats.hp > damaged_hp);
        }
        assert_eq!(game.player.inventory.len(), 0);
    }

    #[test]
    fn test_level_progression() {
        let mut game = Game::new();
        let initial_level = game.level;

        // Place player on stairs
        if let Some(stairs_room) = game.map.rooms.last() {
            game.player.position = stairs_room.center();
        }

        game.descend_stairs();

        assert_eq!(game.level, initial_level + 1);
    }

    #[test]
    fn test_score_calculation() {
        let game = Game::new();
        let score = game.calculate_score();
        assert!(score > 0);
    }

    #[test]
    fn test_inventory_limit() {
        let mut game = Game::new();

        // Clear existing items to have a clean test
        game.items.clear();

        // Fill inventory
        for i in 0..MAX_INVENTORY_SIZE {
            game.player
                .inventory
                .push(Item::new_potion(&format!("Potion {}", i), 10));
        }

        // Try to pick up one more
        game.items
            .push((game.player.position, Item::new_potion("Extra", 10)));
        game.pickup_item();

        assert_eq!(game.player.inventory.len(), MAX_INVENTORY_SIZE);
        assert_eq!(game.items.len(), 1); // Item should still be on ground
    }

    #[test]
    fn test_equipment() {
        let mut game = Game::new();

        let sword = Item::new_weapon("Sword", 5);
        game.player.inventory.push(sword);

        let base_attack = game.player.total_attack();
        game.use_item(0);
        let equipped_attack = game.player.total_attack();

        assert!(equipped_attack > base_attack);
    }

    #[test]
    fn test_fov_updates() {
        let game = Game::new();
        assert!(!game.visible_tiles.is_empty());
        assert!(game.visible_tiles.contains(&game.player.position));
    }
}
