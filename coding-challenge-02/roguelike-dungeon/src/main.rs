mod ai;
mod combat;
mod dungeon;
mod entity;
mod fov;
mod inventory;
mod render;

use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::time::Duration;

use ai::AISystem;
use combat::{CombatLog, CombatSystem};
use dungeon::{Map, TileType};
use entity::{Entity, EntityManager};
use fov::FOV;
use inventory::{Inventory, Item};
use render::Renderer;

/// Game state
struct GameState {
    map: Map,
    entities: EntityManager,
    fov: FOV,
    inventory: Inventory,
    combat_log: CombatLog,
    turn_count: u32,
    game_over: bool,
    victory: bool,
}

impl GameState {
    fn new() -> Self {
        let map = Map::generate_dungeon(80, 50, 1);

        // Spawn player in first room
        let (px, py) = map.rooms[0].center();
        let player = Entity::new_player(px, py);

        let mut entities = EntityManager::new(player);

        // Spawn enemies and items
        Self::populate_dungeon(&map, &mut entities);

        let fov = FOV::new(map.width, map.height);
        let inventory = Inventory::new(10);
        let combat_log = CombatLog::new(50);

        GameState {
            map,
            entities,
            fov,
            inventory,
            combat_log,
            turn_count: 0,
            game_over: false,
            victory: false,
        }
    }

    fn populate_dungeon(map: &Map, entities: &mut EntityManager) {
        let mut rng = rand::thread_rng();

        // Special handling for level 10 (boss level)
        if map.depth >= 10 {
            // Spawn only ONE dragon boss in the last room
            if map.rooms.len() > 1 {
                let last_room = &map.rooms[map.rooms.len() - 1];
                let (x, y) = last_room.center();
                let dragon = Entity::new_enemy(x, y, map.depth);
                entities.enemies.push(dragon);
            }

            // Spawn normal scaled enemies in other rooms (not dragons)
            for room in map.rooms.iter().skip(1).take(map.rooms.len() - 2) {
                let num_enemies = rng.gen_range(1..=2);

                for _ in 0..num_enemies {
                    let x = rng.gen_range(room.x + 1..room.x + room.width - 1);
                    let y = rng.gen_range(room.y + 1..room.y + room.height - 1);

                    if !entities.is_blocked(x, y) {
                        // Create enemy with depth 9 to avoid dragon spawning
                        let enemy = Entity::new_enemy(x, y, 9);
                        entities.enemies.push(enemy);
                    }
                }
            }
        } else {
            // Normal spawning for non-boss levels
            for room in map.rooms.iter().skip(1) {
                let num_enemies = rng.gen_range(1..=3);

                for _ in 0..num_enemies {
                    let x = rng.gen_range(room.x + 1..room.x + room.width - 1);
                    let y = rng.gen_range(room.y + 1..room.y + room.height - 1);

                    if !entities.is_blocked(x, y) {
                        let enemy = Entity::new_enemy(x, y, map.depth);
                        entities.enemies.push(enemy);
                    }
                }

                // Spawn items
                if rng.gen_bool(0.5) {
                    let x = rng.gen_range(room.x + 1..room.x + room.width - 1);
                    let y = rng.gen_range(room.y + 1..room.y + room.height - 1);

                    if !entities.is_blocked(x, y) {
                        let item = Item::random_item(map.depth);
                        entities.items.push((x, y, item));
                    }
                }
            }
        }
    }

    fn next_level(&mut self) {
        // Generate new level
        self.map = Map::generate_dungeon(80, 50, self.map.depth + 1);

        // Place player at start
        let (px, py) = self.map.rooms[0].center();
        self.entities.player.x = px;
        self.entities.player.y = py;

        // Clear old enemies and items
        self.entities.enemies.clear();
        self.entities.items.clear();

        // Populate new level
        Self::populate_dungeon(&self.map, &mut self.entities);

        // Reset FOV
        self.fov = FOV::new(self.map.width, self.map.height);

        // Compute FOV for the new level
        self.fov.compute_fov(&self.map, px, py, 8);

        self.combat_log
            .add_message(format!("Descended to level {}", self.map.depth));

        // Check for victory (reached level 10)
        if self.map.depth >= 10 {
            self.combat_log
                .add_message("You face the Dragon Boss!".to_string());
        }
    }


    fn try_move_player(&mut self, dx: i32, dy: i32) {
        let new_x = self.entities.player.x + dx;
        let new_y = self.entities.player.y + dy;

        // Check for enemy collision - need to check first without borrowing
        let has_enemy = self.entities.enemies.iter().any(|e| e.x == new_x && e.y == new_y && e.is_alive());

        if has_enemy {
            let attack_bonus = self.inventory.get_attack_bonus();
            let mut player_with_bonus = self.entities.player.clone();
            player_with_bonus.attack += attack_bonus;

            // Now we can safely get mutable access
            if let Some(enemy) = self.entities.get_enemy_at(new_x, new_y) {
                let result = CombatSystem::attack(&player_with_bonus, enemy);
                self.combat_log.add_combat_result(&result);

                if result.defender_died {
                    let xp = enemy.xp;
                    if self.entities.player.gain_xp(xp) {
                        self.combat_log.add_message(format!(
                            "{} leveled up to level {}!",
                            self.entities.player.name, self.entities.player.level
                        ));
                    }
                    self.entities.remove_dead();
                }
            }

            self.turn_count += 1;
            return;
        }

        // Check if move is valid
        if self.map.is_walkable(new_x, new_y) && !self.entities.is_blocked(new_x, new_y) {
            self.entities.player.x = new_x;
            self.entities.player.y = new_y;
            self.turn_count += 1;
        }
    }

    fn pickup_item(&mut self) {
        let px = self.entities.player.x;
        let py = self.entities.player.y;

        if let Some(item) = self.entities.remove_item_at(px, py) {
            if self.inventory.add_item(item.clone()) {
                self.combat_log
                    .add_message(format!("Picked up {}", item.name));
            } else {
                self.combat_log
                    .add_message("Inventory is full!".to_string());
                self.entities.items.push((px, py, item));
            }
        } else {
            self.combat_log
                .add_message("Nothing to pick up here.".to_string());
        }
    }

    fn use_stairs_down(&mut self) {
        let px = self.entities.player.x;
        let py = self.entities.player.y;
        let idx = self.map.xy_idx(px, py);

        if self.map.tiles[idx] == TileType::StairsDown {
            // Check if all enemies are dead
            let alive_enemies = self.entities.enemies.iter().filter(|e| e.is_alive()).count();

            if alive_enemies > 0 {
                self.combat_log.add_message(
                    "You must defeat all enemies before descending!".to_string(),
                );
                return;
            }

            if self.map.depth >= 10 {
                // Check if dragon is defeated
                let dragon_alive = self
                    .entities
                    .enemies
                    .iter()
                    .any(|e| e.is_alive() && matches!(e.ai_type, Some(entity::AIType::Dragon)));

                if !dragon_alive {
                    self.victory = true;
                    self.game_over = true;
                    return;
                }
            }

            self.next_level();
        } else {
            self.combat_log
                .add_message("There are no stairs here.".to_string());
        }
    }


    fn update_enemies(&mut self) {
        let moves = AISystem::update_enemies(&mut self.entities, &self.map, &self.fov, self.turn_count);

        for (idx, dx, dy) in moves {
            let enemy = &self.entities.enemies[idx];
            let new_x = enemy.x + dx;
            let new_y = enemy.y + dy;

            // Check if attacking player
            if new_x == self.entities.player.x && new_y == self.entities.player.y {
                let mut player_clone = self.entities.player.clone();
                let enemy_clone = enemy.clone();
                let defense_bonus = self.inventory.get_defense_bonus();
                player_clone.defense += defense_bonus;

                let result = CombatSystem::attack(&enemy_clone, &mut player_clone);
                self.combat_log.add_combat_result(&result);

                self.entities.player = player_clone;

                if result.defender_died {
                    self.game_over = true;
                }
            } else if self.map.is_walkable(new_x, new_y)
                && !self.entities.is_blocked(new_x, new_y)
            {
                // Move enemy
                self.entities.enemies[idx].x = new_x;
                self.entities.enemies[idx].y = new_y;
            }
        }
    }

    fn handle_inventory(&mut self, renderer: &Renderer) -> std::io::Result<()> {
        loop {
            renderer.render_inventory(&self.inventory)?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('e') | KeyCode::Esc => break,
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            let num = c.to_digit(10).unwrap() as usize;
                            if num > 0 && num <= self.inventory.items.len() {
                                let idx = num - 1;
                                let item = self.inventory.items[idx].clone();

                                if item.consumable {
                                    if let Some(used_item) = self.inventory.use_item(idx) {
                                        match used_item.item_type {
                                            inventory::ItemType::HealthPotion => {
                                                self.entities.player.heal(used_item.value);
                                                self.combat_log.add_message(format!(
                                                    "Used {}. Healed {} HP",
                                                    used_item.name, used_item.value
                                                ));
                                            }
                                            inventory::ItemType::ManaPotion => {
                                                self.combat_log.add_message(format!(
                                                    "Used {} (no effect yet)",
                                                    used_item.name
                                                ));
                                            }
                                            _ => {}
                                        }
                                    }
                                } else {
                                    if let Some(msg) = self.inventory.equip_item(idx) {
                                        self.combat_log.add_message(msg);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new()?;
    let mut game = GameState::new();

    // Main game loop
    loop {
        // Update FOV
        game.fov.compute_fov(
            &game.map,
            game.entities.player.x,
            game.entities.player.y,
            8,
        );

        // Render
        renderer.render(
            &game.map,
            &game.entities,
            &game.fov,
            &game.inventory,
            &game.combat_log,
        )?;

        // Check for game over
        if game.game_over {
            renderer.render_game_over(
                game.victory,
                game.map.depth,
                game.entities.player.level,
            )?;
            event::read()?;
            break;
        }

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                let mut took_turn = false;

                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('w') | KeyCode::Up => {
                        game.try_move_player(0, -1);
                        took_turn = true;
                    }
                    KeyCode::Char('s') | KeyCode::Down => {
                        game.try_move_player(0, 1);
                        took_turn = true;
                    }
                    KeyCode::Char('a') | KeyCode::Left => {
                        game.try_move_player(-1, 0);
                        took_turn = true;
                    }
                    KeyCode::Char('d') | KeyCode::Right => {
                        game.try_move_player(1, 0);
                        took_turn = true;
                    }
                    KeyCode::Char('g') => {
                        game.pickup_item();
                        took_turn = true;
                    }
                    KeyCode::Char('>') => {
                        game.use_stairs_down();
                        took_turn = true;
                    }
                    KeyCode::Char('i') => {
                        game.handle_inventory(&renderer)?;
                    }
                    _ => {}
                }

                // Update enemies if player took a turn
                if took_turn {
                    game.update_enemies();
                }
            }
        }
    }

    renderer.cleanup()?;
    Ok(())
}
