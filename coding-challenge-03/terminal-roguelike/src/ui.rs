//! Terminal UI rendering with beautiful ASCII art

use crate::game::Game;
use crate::map::MAP_HEIGHT;
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Write};

const MESSAGE_HEIGHT: u16 = 8;
#[allow(dead_code)]
const STATS_WIDTH: u16 = 30;

pub fn render(game: &Game) -> io::Result<()> {
    let mut stdout = stdout();

    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    render_map(game, &mut stdout)?;
    render_stats(game, &mut stdout)?;
    render_messages(game, &mut stdout)?;
    render_help(&mut stdout)?;

    if game.is_game_over() {
        render_game_over(game, &mut stdout)?;
    }

    stdout.flush()?;

    Ok(())
}

fn render_map(game: &Game, stdout: &mut impl Write) -> io::Result<()> {
    for y in 0..game.map.height {
        stdout.queue(cursor::MoveTo(0, y as u16))?;

        for x in 0..game.map.width {
            let pos = crate::entity::Position::new(x, y);

            if game.visible_tiles.contains(&pos) {
                // Render visible entities first
                if pos == game.player.position {
                    let (r, g, b) = game.player.color();
                    stdout.queue(SetForegroundColor(Color::Rgb { r, g, b }))?;
                    stdout.queue(Print(game.player.symbol()))?;
                    stdout.queue(ResetColor)?;
                } else if let Some(enemy) = game
                    .enemies
                    .iter()
                    .find(|e| e.position == pos && e.is_alive())
                {
                    let (r, g, b) = enemy.color();
                    stdout.queue(SetForegroundColor(Color::Rgb { r, g, b }))?;
                    stdout.queue(Print(enemy.symbol()))?;
                    stdout.queue(ResetColor)?;
                } else if let Some((_, item)) = game.items.iter().find(|(p, _)| *p == pos) {
                    let (r, g, b) = item.color();
                    stdout.queue(SetForegroundColor(Color::Rgb { r, g, b }))?;
                    stdout.queue(Print(item.symbol()))?;
                    stdout.queue(ResetColor)?;
                } else if let Some(tile) = game.map.get_tile(x, y) {
                    let (r, g, b) = tile.color();
                    stdout.queue(SetForegroundColor(Color::Rgb { r, g, b }))?;
                    stdout.queue(Print(tile.symbol()))?;
                    stdout.queue(ResetColor)?;
                } else {
                    stdout.queue(Print(' '))?;
                }
            } else if game.map.is_revealed(x, y) {
                // Render revealed but not visible (fog of war)
                if let Some(tile) = game.map.get_tile(x, y) {
                    stdout.queue(SetForegroundColor(Color::DarkGrey))?;
                    stdout.queue(Print(tile.symbol()))?;
                    stdout.queue(ResetColor)?;
                } else {
                    stdout.queue(Print(' '))?;
                }
            } else {
                stdout.queue(Print(' '))?;
            }
        }
    }

    Ok(())
}

fn render_stats(game: &Game, stdout: &mut impl Write) -> io::Result<()> {
    let stats_x = game.map.width as u16 + 2;
    let mut y = 1u16;

    // Title
    stdout.queue(cursor::MoveTo(stats_x, y))?;
    stdout.queue(SetForegroundColor(Color::Yellow))?;
    stdout.queue(Print("=== STATS ==="))?;
    stdout.queue(ResetColor)?;
    y += 2;

    // Player stats
    if let Some(stats) = &game.player.stats {
        stdout.queue(cursor::MoveTo(stats_x, y))?;
        stdout.queue(Print(format!("Level: {}", stats.level)))?;
        y += 1;

        stdout.queue(cursor::MoveTo(stats_x, y))?;
        stdout.queue(Print(format!("HP: {}/{}", stats.hp, stats.max_hp)))?;
        y += 1;

        // HP bar
        stdout.queue(cursor::MoveTo(stats_x, y))?;
        let hp_percent = stats.hp as f32 / stats.max_hp as f32;
        let bar_width = 20;
        let filled = (hp_percent * bar_width as f32) as usize;

        let color = if hp_percent > 0.6 {
            Color::Green
        } else if hp_percent > 0.3 {
            Color::Yellow
        } else {
            Color::Red
        };

        stdout.queue(SetForegroundColor(color))?;
        stdout.queue(Print("HP ["))?;
        for i in 0..bar_width {
            if i < filled {
                stdout.queue(Print("="))?;
            } else {
                stdout.queue(Print(" "))?;
            }
        }
        stdout.queue(Print("]"))?;
        stdout.queue(ResetColor)?;
        y += 2;

        stdout.queue(cursor::MoveTo(stats_x, y))?;
        stdout.queue(Print(format!("Attack: {}", game.player.total_attack())))?;
        y += 1;

        stdout.queue(cursor::MoveTo(stats_x, y))?;
        stdout.queue(Print(format!("Defense: {}", game.player.total_defense())))?;
        y += 1;

        stdout.queue(cursor::MoveTo(stats_x, y))?;
        stdout.queue(Print(format!(
            "XP: {}/{}",
            stats.xp,
            stats.xp_to_next_level()
        )))?;
        y += 2;
    }

    stdout.queue(cursor::MoveTo(stats_x, y))?;
    stdout.queue(Print(format!("Dungeon Level: {}", game.level)))?;
    y += 2;

    // Equipment
    stdout.queue(cursor::MoveTo(stats_x, y))?;
    stdout.queue(SetForegroundColor(Color::Cyan))?;
    stdout.queue(Print("=== EQUIPMENT ==="))?;
    stdout.queue(ResetColor)?;
    y += 1;

    stdout.queue(cursor::MoveTo(stats_x, y))?;
    if let Some(weapon) = &game.player.equipped_weapon {
        stdout.queue(Print(format!("Weapon: {}", weapon.name)))?;
    } else {
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        stdout.queue(Print("Weapon: None"))?;
        stdout.queue(ResetColor)?;
    }
    y += 1;

    stdout.queue(cursor::MoveTo(stats_x, y))?;
    if let Some(armor) = &game.player.equipped_armor {
        stdout.queue(Print(format!("Armor: {}", armor.name)))?;
    } else {
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        stdout.queue(Print("Armor: None"))?;
        stdout.queue(ResetColor)?;
    }
    y += 2;

    // Inventory
    stdout.queue(cursor::MoveTo(stats_x, y))?;
    stdout.queue(SetForegroundColor(Color::Magenta))?;
    stdout.queue(Print("=== INVENTORY ==="))?;
    stdout.queue(ResetColor)?;
    y += 1;

    for (i, item) in game.player.inventory.iter().enumerate() {
        if i >= 9 {
            break;
        }
        stdout.queue(cursor::MoveTo(stats_x, y))?;
        stdout.queue(Print(format!("{}: {}", i + 1, item.name)))?;
        y += 1;
    }

    if game.player.inventory.is_empty() {
        stdout.queue(cursor::MoveTo(stats_x, y))?;
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        stdout.queue(Print("(empty)"))?;
        stdout.queue(ResetColor)?;
    }

    Ok(())
}

fn render_messages(game: &Game, stdout: &mut impl Write) -> io::Result<()> {
    let messages_y = MAP_HEIGHT as u16;
    let mut y = messages_y + 1;

    stdout.queue(cursor::MoveTo(0, messages_y))?;
    stdout.queue(SetForegroundColor(Color::Yellow))?;
    stdout.queue(Print("=".repeat(80)))?;
    stdout.queue(ResetColor)?;

    let start_idx = if game.messages.len() > MESSAGE_HEIGHT as usize {
        game.messages.len() - MESSAGE_HEIGHT as usize
    } else {
        0
    };

    for msg in &game.messages[start_idx..] {
        stdout.queue(cursor::MoveTo(0, y))?;
        stdout.queue(Print(msg))?;
        y += 1;

        if y >= messages_y + MESSAGE_HEIGHT {
            break;
        }
    }

    Ok(())
}

fn render_help(stdout: &mut impl Write) -> io::Result<()> {
    let y = MAP_HEIGHT as u16 + MESSAGE_HEIGHT + 2;

    stdout.queue(cursor::MoveTo(0, y))?;
    stdout.queue(SetForegroundColor(Color::DarkGrey))?;
    stdout.queue(Print(
        "Arrow keys/hjkl: Move | g: Pickup | 1-9: Use item | >: Descend stairs | q: Quit",
    ))?;
    stdout.queue(ResetColor)?;

    Ok(())
}

fn render_game_over(game: &Game, stdout: &mut impl Write) -> io::Result<()> {
    let (term_width, term_height) = terminal::size()?;

    let box_width = 50u16;
    let box_height = 15u16;
    let box_x = (term_width - box_width) / 2;
    let box_y = (term_height - box_height) / 2;

    // Draw box
    for y in 0..box_height {
        stdout.queue(cursor::MoveTo(box_x, box_y + y))?;
        stdout.queue(SetForegroundColor(Color::Red))?;

        if y == 0 || y == box_height - 1 {
            stdout.queue(Print("=".repeat(box_width as usize)))?;
        } else {
            stdout.queue(Print("="))?;
            stdout.queue(cursor::MoveTo(box_x + box_width - 1, box_y + y))?;
            stdout.queue(Print("="))?;
        }

        stdout.queue(ResetColor)?;
    }

    // Game over text
    let mut y = box_y + 2;
    stdout.queue(cursor::MoveTo(box_x + 15, y))?;
    stdout.queue(SetForegroundColor(Color::Red))?;
    stdout.queue(Print("GAME OVER"))?;
    stdout.queue(ResetColor)?;
    y += 2;

    if let Some(stats) = &game.player.stats {
        stdout.queue(cursor::MoveTo(box_x + 10, y))?;
        stdout.queue(Print(format!("Final Level: {}", stats.level)))?;
        y += 1;

        stdout.queue(cursor::MoveTo(box_x + 10, y))?;
        stdout.queue(Print(format!("Dungeon Depth: {}", game.level)))?;
        y += 1;

        let score = game.level * 100 + stats.xp + (stats.level - 1) * 100;
        stdout.queue(cursor::MoveTo(box_x + 10, y))?;
        stdout.queue(Print(format!("Final Score: {}", score)))?;
        y += 2;
    }

    // High scores
    stdout.queue(cursor::MoveTo(box_x + 14, y))?;
    stdout.queue(SetForegroundColor(Color::Yellow))?;
    stdout.queue(Print("HIGH SCORES"))?;
    stdout.queue(ResetColor)?;
    y += 1;

    for (i, score) in game.high_scores.get_scores().iter().take(3).enumerate() {
        stdout.queue(cursor::MoveTo(box_x + 10, y))?;
        stdout.queue(Print(format!(
            "{}. Level {} - Score: {}",
            i + 1,
            score.level,
            score.score
        )))?;
        y += 1;
    }

    y += 1;
    stdout.queue(cursor::MoveTo(box_x + 8, y))?;
    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.queue(Print("Press 'n' for New Game or 'q' to Quit"))?;
    stdout.queue(ResetColor)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_height_constant() {
        assert!(MESSAGE_HEIGHT > 0);
        assert!(MESSAGE_HEIGHT < 20);
    }

    #[test]
    fn test_stats_width_constant() {
        assert!(STATS_WIDTH > 0);
        assert!(STATS_WIDTH < 50);
    }
}
