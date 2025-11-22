use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

use crate::combat::CombatLog;
use crate::dungeon::{Map, TileType};
use crate::entity::EntityManager;
use crate::fov::FOV;
use crate::inventory::Inventory;

/// Rendering system for the terminal
pub struct Renderer {
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), terminal::EnterAlternateScreen)?;

        let (width, height) = terminal::size()?;

        Ok(Renderer { width, height })
    }

    pub fn cleanup(&self) -> std::io::Result<()> {
        execute!(stdout(), terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Clear the screen
    pub fn clear(&self) -> std::io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        Ok(())
    }

    /// Render the entire game state
    pub fn render(
        &self,
        map: &Map,
        entities: &EntityManager,
        fov: &FOV,
        inventory: &Inventory,
        combat_log: &CombatLog,
    ) -> std::io::Result<()> {
        self.clear()?;

        // Calculate viewport
        let map_width = 60;
        let map_height = 40;

        // Center view on player
        let view_x = (entities.player.x - map_width / 2).max(0).min(map.width - map_width);
        let view_y = (entities.player.y - map_height / 2).max(0).min(map.height - map_height);

        // Render map
        for y in 0..map_height {
            for x in 0..map_width {
                let map_x = view_x + x;
                let map_y = view_y + y;

                if !map.in_bounds(map_x, map_y) {
                    continue;
                }

                execute!(stdout(), cursor::MoveTo(x as u16, y as u16))?;

                if fov.is_visible(map_x, map_y) {
                    self.render_tile_visible(map, map_x, map_y)?;
                } else if fov.is_explored(map_x, map_y) {
                    self.render_tile_explored(map, map_x, map_y)?;
                } else {
                    execute!(stdout(), Print(" "))?;
                }
            }
        }

        // Render items
        for (item_x, item_y, item) in &entities.items {
            let screen_x = item_x - view_x;
            let screen_y = item_y - view_y;

            if screen_x >= 0
                && screen_x < map_width
                && screen_y >= 0
                && screen_y < map_height
                && fov.is_visible(*item_x, *item_y)
            {
                execute!(
                    stdout(),
                    cursor::MoveTo(screen_x as u16, screen_y as u16),
                    SetForegroundColor(Color::Yellow),
                    Print(item.symbol),
                    ResetColor
                )?;
            }
        }

        // Render enemies
        for enemy in &entities.enemies {
            if !enemy.is_alive() {
                continue;
            }

            let screen_x = enemy.x - view_x;
            let screen_y = enemy.y - view_y;

            if screen_x >= 0
                && screen_x < map_width
                && screen_y >= 0
                && screen_y < map_height
                && fov.is_visible(enemy.x, enemy.y)
            {
                let color = match enemy.ai_type {
                    Some(crate::entity::AIType::Zombie) => Color::Green,
                    Some(crate::entity::AIType::Goblin) => Color::Red,
                    Some(crate::entity::AIType::Orc) => Color::DarkRed,
                    Some(crate::entity::AIType::Demon) => Color::Magenta,
                    Some(crate::entity::AIType::Dragon) => Color::DarkYellow,
                    None => Color::White,
                };

                execute!(
                    stdout(),
                    cursor::MoveTo(screen_x as u16, screen_y as u16),
                    SetForegroundColor(color),
                    Print(enemy.symbol),
                    ResetColor
                )?;
            }
        }

        // Render player
        let player_screen_x = entities.player.x - view_x;
        let player_screen_y = entities.player.y - view_y;
        execute!(
            stdout(),
            cursor::MoveTo(player_screen_x as u16, player_screen_y as u16),
            SetForegroundColor(Color::Cyan),
            Print(entities.player.symbol),
            ResetColor
        )?;

        // Render UI
        self.render_ui(entities, inventory, combat_log, map)?;

        stdout().flush()?;
        Ok(())
    }

    fn render_tile_visible(&self, map: &Map, x: i32, y: i32) -> std::io::Result<()> {
        let idx = map.xy_idx(x, y);
        match map.tiles[idx] {
            TileType::Wall => {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::Grey),
                    Print('#'),
                    ResetColor
                )?;
            }
            TileType::Floor => {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::DarkGrey),
                    Print('.'),
                    ResetColor
                )?;
            }
            TileType::StairsDown => {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::White),
                    Print('>'),
                    ResetColor
                )?;
            }
            TileType::StairsUp => {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::White),
                    Print('<'),
                    ResetColor
                )?;
            }
        }
        Ok(())
    }

    fn render_tile_explored(&self, map: &Map, x: i32, y: i32) -> std::io::Result<()> {
        let idx = map.xy_idx(x, y);
        match map.tiles[idx] {
            TileType::Wall => {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::DarkGrey),
                    Print('#'),
                    ResetColor
                )?;
            }
            TileType::Floor => {
                execute!(stdout(), Print(' '))?;
            }
            TileType::StairsDown => {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::DarkGrey),
                    Print('>'),
                    ResetColor
                )?;
            }
            TileType::StairsUp => {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::DarkGrey),
                    Print('<'),
                    ResetColor
                )?;
            }
        }
        Ok(())
    }

    fn render_ui(
        &self,
        entities: &EntityManager,
        inventory: &Inventory,
        combat_log: &CombatLog,
        map: &Map,
    ) -> std::io::Result<()> {
        let ui_x = 62;
        let ui_y = 0;

        // Player stats
        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y),
            SetForegroundColor(Color::White),
            Print(format!("=== {} ===", entities.player.name)),
            ResetColor
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 1),
            SetForegroundColor(Color::Red),
            Print(format!("HP: {}/{}", entities.player.hp, entities.player.max_hp)),
            ResetColor
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 2),
            SetForegroundColor(Color::Green),
            Print(format!("Level: {}", entities.player.level)),
            ResetColor
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 3),
            SetForegroundColor(Color::Yellow),
            Print(format!(
                "XP: {}/{}",
                entities.player.xp,
                entities.player.xp_to_next_level()
            )),
            ResetColor
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 4),
            Print(format!("Attack: {}", entities.player.attack + inventory.get_attack_bonus()))
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 5),
            Print(format!(
                "Defense: {}",
                entities.player.defense + inventory.get_defense_bonus()
            ))
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 6),
            SetForegroundColor(Color::Magenta),
            Print(format!("Dungeon Level: {}", map.depth)),
            ResetColor
        )?;

        // Inventory
        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 8),
            SetForegroundColor(Color::White),
            Print("=== Inventory ==="),
            ResetColor
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 9),
            Print(format!("Items: {}/{}", inventory.count(), inventory.max_size))
        )?;

        // Equipment
        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 11),
            SetForegroundColor(Color::White),
            Print("=== Equipment ==="),
            ResetColor
        )?;

        let weapon_name = inventory
            .equipped_weapon
            .as_ref()
            .map(|w| w.name.as_str())
            .unwrap_or("None");
        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 12),
            Print(format!("Weapon: {}", weapon_name))
        )?;

        let armor_name = inventory
            .equipped_armor
            .as_ref()
            .map(|a| a.name.as_str())
            .unwrap_or("None");
        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 13),
            Print(format!("Armor: {}", armor_name))
        )?;

        let shield_name = inventory
            .equipped_shield
            .as_ref()
            .map(|s| s.name.as_str())
            .unwrap_or("None");
        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 14),
            Print(format!("Shield: {}", shield_name))
        )?;

        // Combat log
        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 16),
            SetForegroundColor(Color::White),
            Print("=== Combat Log ==="),
            ResetColor
        )?;

        let recent_messages = combat_log.get_recent(8);
        for (i, message) in recent_messages.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(ui_x, ui_y + 17 + i as u16),
                Print(format!("{}", message))
            )?;
        }

        // Controls
        execute!(
            stdout(),
            cursor::MoveTo(ui_x, ui_y + 27),
            SetForegroundColor(Color::White),
            Print("=== Controls ==="),
            ResetColor
        )?;

        let controls = vec![
            "WASD/Arrows: Move",
            "I: Inventory",
            "G: Pick up item",
            ">: Descend stairs",
            "<: Ascend stairs",
            "Q: Quit",
        ];

        for (i, control) in controls.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(ui_x, ui_y + 28 + i as u16),
                Print(control)
            )?;
        }

        Ok(())
    }

    /// Render inventory screen
    pub fn render_inventory(&self, inventory: &Inventory) -> std::io::Result<()> {
        self.clear()?;

        execute!(
            stdout(),
            cursor::MoveTo(2, 2),
            SetForegroundColor(Color::White),
            Print("=== INVENTORY ==="),
            ResetColor
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(2, 4),
            Print(format!("Items: {}/{}", inventory.count(), inventory.max_size))
        )?;

        for (i, item) in inventory.items.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(2, 6 + i as u16),
                Print(format!("{}: {} {}", i + 1, item.symbol, item.name))
            )?;
        }

        execute!(
            stdout(),
            cursor::MoveTo(2, 16),
            SetForegroundColor(Color::Yellow),
            Print("1-9: Use/Equip item | E: Close inventory"),
            ResetColor
        )?;

        stdout().flush()?;
        Ok(())
    }

    /// Render game over screen
    pub fn render_game_over(&self, victory: bool, depth: i32, level: i32) -> std::io::Result<()> {
        self.clear()?;

        let (title, color) = if victory {
            ("=== VICTORY! ===", Color::Green)
        } else {
            ("=== GAME OVER ===", Color::Red)
        };

        execute!(
            stdout(),
            cursor::MoveTo(self.width / 2 - 10, self.height / 2 - 3),
            SetForegroundColor(color),
            Print(title),
            ResetColor
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(self.width / 2 - 10, self.height / 2 - 1),
            Print(format!("Reached Depth: {}", depth))
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(self.width / 2 - 10, self.height / 2),
            Print(format!("Final Level: {}", level))
        )?;

        execute!(
            stdout(),
            cursor::MoveTo(self.width / 2 - 10, self.height / 2 + 2),
            SetForegroundColor(Color::Yellow),
            Print("Press any key to exit..."),
            ResetColor
        )?;

        stdout().flush()?;
        Ok(())
    }
}
