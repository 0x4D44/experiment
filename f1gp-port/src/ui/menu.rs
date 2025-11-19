//! Menu system
//!
//! Provides menu UI for game navigation.

use crate::game::weather::WeatherCondition;
use crate::platform::{Color, Rect, Renderer};
use anyhow::Result;
use glam::Vec2;

/// Menu item selection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    /// No action
    None,

    /// Start a new race
    StartRace,

    /// Show race setup screen
    RaceSetup,

    /// Resume game (from pause)
    Resume,

    /// Restart race
    Restart,

    /// Return to main menu
    MainMenu,

    /// Show options
    Options,

    /// Exit game
    Exit,
}

/// Menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Display text
    pub text: String,

    /// Action when selected
    pub action: MenuAction,

    /// Is this item enabled?
    pub enabled: bool,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(text: impl Into<String>, action: MenuAction) -> Self {
        Self {
            text: text.into(),
            action,
            enabled: true,
        }
    }

    /// Create a disabled menu item
    pub fn disabled(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            action: MenuAction::None,
            enabled: false,
        }
    }
}

/// Menu type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuType {
    /// Main menu
    Main,

    /// Race setup menu
    RaceSetup,

    /// Pause menu
    Pause,

    /// Race results
    Results,
}

/// Menu UI
pub struct Menu {
    /// Current menu type
    pub menu_type: MenuType,

    /// Menu items
    items: Vec<MenuItem>,

    /// Currently selected item index
    selected_index: usize,

    /// Menu title
    title: String,

    /// Screen dimensions
    screen_width: u32,
    screen_height: u32,
}

impl Menu {
    /// Create main menu
    pub fn main_menu(screen_width: u32, screen_height: u32) -> Self {
        let items = vec![
            MenuItem::new("START RACE", MenuAction::StartRace),
            MenuItem::new("RACE SETUP", MenuAction::RaceSetup),
            MenuItem::new("OPTIONS", MenuAction::Options),
            MenuItem::new("EXIT", MenuAction::Exit),
        ];

        Self {
            menu_type: MenuType::Main,
            items,
            selected_index: 0,
            title: "F1GP MODERN PORT".to_string(),
            screen_width,
            screen_height,
        }
    }

    /// Create race setup menu
    pub fn race_setup_menu(screen_width: u32, screen_height: u32, num_opponents: usize, weather: WeatherCondition) -> Self {
        let weather_text = match weather {
            WeatherCondition::Dry => "DRY",
            WeatherCondition::LightRain => "LIGHT RAIN",
            WeatherCondition::HeavyRain => "HEAVY RAIN",
        };

        let items = vec![
            MenuItem::new(format!("OPPONENTS: {}", num_opponents), MenuAction::None),
            MenuItem::new(format!("WEATHER: {}", weather_text), MenuAction::None),
            MenuItem::new("START RACE", MenuAction::StartRace),
            MenuItem::new("BACK", MenuAction::MainMenu),
        ];

        Self {
            menu_type: MenuType::RaceSetup,
            items,
            selected_index: 2, // Default to "Start Race"
            title: "RACE SETUP".to_string(),
            screen_width,
            screen_height,
        }
    }

    /// Create pause menu
    pub fn pause_menu(screen_width: u32, screen_height: u32) -> Self {
        let items = vec![
            MenuItem::new("RESUME", MenuAction::Resume),
            MenuItem::new("RESTART", MenuAction::Restart),
            MenuItem::new("MAIN MENU", MenuAction::MainMenu),
        ];

        Self {
            menu_type: MenuType::Pause,
            items,
            selected_index: 0,
            title: "PAUSED".to_string(),
            screen_width,
            screen_height,
        }
    }

    /// Create results menu
    pub fn results_menu(screen_width: u32, screen_height: u32) -> Self {
        let items = vec![
            MenuItem::new("RESTART", MenuAction::Restart),
            MenuItem::new("MAIN MENU", MenuAction::MainMenu),
        ];

        Self {
            menu_type: MenuType::Results,
            items,
            selected_index: 0,
            title: "RACE FINISHED".to_string(),
            screen_width,
            screen_height,
        }
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;

            // Skip disabled items
            while !self.items[self.selected_index].enabled && self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;

            // Skip disabled items
            while self.selected_index < self.items.len()
                && !self.items[self.selected_index].enabled {
                self.selected_index += 1;
            }

            // Clamp to valid range
            if self.selected_index >= self.items.len() {
                self.selected_index = self.items.len() - 1;
            }
        }
    }

    /// Get selected action
    pub fn get_selected_action(&self) -> MenuAction {
        self.items.get(self.selected_index)
            .map(|item| item.action)
            .unwrap_or(MenuAction::None)
    }

    /// Update menu item text (e.g., for changing number of opponents)
    pub fn update_item_text(&mut self, index: usize, text: String) {
        if let Some(item) = self.items.get_mut(index) {
            item.text = text;
        }
    }

    /// Get currently selected menu item index
    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }

    /// Render the menu
    pub fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        let center_x = self.screen_width as f32 / 2.0;
        let center_y = self.screen_height as f32 / 2.0;

        // Draw semi-transparent background
        renderer.draw_filled_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                width: self.screen_width as f32,
                height: self.screen_height as f32,
            },
            Color::rgba(0, 0, 0, 180),
        )?;

        // Draw title
        let title_size = 32.0;
        let title_y = center_y - 150.0;
        renderer.draw_text(
            &self.title,
            Vec2::new(center_x - (self.title.len() as f32 * title_size * 0.3), title_y),
            title_size,
            Color::WHITE,
        )?;

        // Draw menu items
        let item_size = 20.0;
        let item_spacing = 40.0;
        let start_y = center_y - (self.items.len() as f32 * item_spacing / 2.0);

        for (i, item) in self.items.iter().enumerate() {
            let y = start_y + (i as f32 * item_spacing);
            let is_selected = i == self.selected_index;

            // Determine color
            let color = if !item.enabled {
                Color::rgba(100, 100, 100, 255)
            } else if is_selected {
                Color::rgba(255, 255, 0, 255) // Yellow for selected
            } else {
                Color::WHITE
            };

            // Draw selection indicator
            if is_selected && item.enabled {
                renderer.draw_text(
                    ">",
                    Vec2::new(center_x - (item.text.len() as f32 * item_size * 0.35) - 30.0, y),
                    item_size,
                    color,
                )?;
            }

            // Draw menu item text
            renderer.draw_text(
                &item.text,
                Vec2::new(center_x - (item.text.len() as f32 * item_size * 0.3), y),
                item_size,
                color,
            )?;
        }

        // Draw controls hint
        let hint_y = self.screen_height as f32 - 40.0;
        let hint_size = 12.0;
        let hint_text = "UP/DOWN: Navigate  ENTER: Select  ESC: Back";
        renderer.draw_text(
            hint_text,
            Vec2::new(center_x - (hint_text.len() as f32 * hint_size * 0.3), hint_y),
            hint_size,
            Color::rgba(200, 200, 200, 255),
        )?;

        Ok(())
    }

    /// Resize menu
    pub fn resize(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_creation() {
        let menu = Menu::main_menu(800, 600);
        assert_eq!(menu.menu_type, MenuType::Main);
        assert_eq!(menu.items.len(), 4);
        assert_eq!(menu.selected_index, 0);
    }

    #[test]
    fn test_menu_navigation() {
        let mut menu = Menu::main_menu(800, 600);

        // Start at index 0
        assert_eq!(menu.selected_index, 0);

        // Move down
        menu.move_down();
        assert_eq!(menu.selected_index, 1);

        // Move up
        menu.move_up();
        assert_eq!(menu.selected_index, 0);

        // Try to move up past first item
        menu.move_up();
        assert_eq!(menu.selected_index, 0);
    }

    #[test]
    fn test_menu_action() {
        let menu = Menu::main_menu(800, 600);
        let action = menu.get_selected_action();
        assert_eq!(action, MenuAction::StartRace);
    }

    #[test]
    fn test_pause_menu() {
        let menu = Menu::pause_menu(800, 600);
        assert_eq!(menu.menu_type, MenuType::Pause);
        assert_eq!(menu.get_selected_action(), MenuAction::Resume);
    }

    #[test]
    fn test_skip_disabled_items() {
        let mut menu = Menu::main_menu(800, 600);
        menu.items[1].enabled = false; // Disable second item

        menu.move_down(); // Should skip disabled item
        assert_eq!(menu.selected_index, 2); // Should jump to third item
    }
}
