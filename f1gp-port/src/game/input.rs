//! Input handling system
//!
//! Manages keyboard and gamepad input for car controls.

use sdl2::keyboard::Keycode;
use std::collections::HashSet;

/// Input state for a car
#[derive(Debug, Clone, Default)]
pub struct CarInput {
    /// Throttle (0.0-1.0)
    pub throttle: f32,

    /// Brake (0.0-1.0)
    pub brake: f32,

    /// Steering (-1.0 to 1.0, left to right)
    pub steering: f32,

    /// Shift up requested
    pub shift_up: bool,

    /// Shift down requested
    pub shift_down: bool,
}

impl CarInput {
    /// Create new default input state
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset all inputs to zero
    pub fn reset(&mut self) {
        self.throttle = 0.0;
        self.brake = 0.0;
        self.steering = 0.0;
        self.shift_up = false;
        self.shift_down = false;
    }
}

/// Input manager handles keyboard state
#[derive(Debug)]
pub struct InputManager {
    /// Set of currently pressed keys
    pressed_keys: HashSet<Keycode>,

    /// Previous frame's pressed keys (for edge detection)
    previous_keys: HashSet<Keycode>,
}

impl InputManager {
    /// Create a new input manager
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            previous_keys: HashSet::new(),
        }
    }

    /// Update input state for new frame
    pub fn update(&mut self) {
        self.previous_keys = self.pressed_keys.clone();
    }

    /// Handle key press
    pub fn key_down(&mut self, keycode: Keycode) {
        self.pressed_keys.insert(keycode);
    }

    /// Handle key release
    pub fn key_up(&mut self, keycode: Keycode) {
        self.pressed_keys.remove(&keycode);
    }

    /// Check if key is currently pressed
    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    /// Check if key was just pressed this frame
    pub fn is_key_just_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode) && !self.previous_keys.contains(&keycode)
    }

    /// Get car input from keyboard state
    pub fn get_car_input(&self) -> CarInput {
        let mut input = CarInput::new();

        // Throttle (Up arrow or W)
        if self.is_key_pressed(Keycode::Up) || self.is_key_pressed(Keycode::W) {
            input.throttle = 1.0;
        }

        // Brake (Down arrow or S)
        if self.is_key_pressed(Keycode::Down) || self.is_key_pressed(Keycode::S) {
            input.brake = 1.0;
        }

        // Steering (Left/Right arrows or A/D)
        if self.is_key_pressed(Keycode::Left) || self.is_key_pressed(Keycode::A) {
            input.steering = -1.0;
        } else if self.is_key_pressed(Keycode::Right) || self.is_key_pressed(Keycode::D) {
            input.steering = 1.0;
        }

        // Gear shifts (Z for down, X for up)
        input.shift_up = self.is_key_just_pressed(Keycode::X);
        input.shift_down = self.is_key_just_pressed(Keycode::Z);

        input
    }

    /// Clear all input state
    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.previous_keys.clear();
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_manager_creation() {
        let manager = InputManager::new();
        assert!(!manager.is_key_pressed(Keycode::Up));
    }

    #[test]
    fn test_key_press() {
        let mut manager = InputManager::new();
        manager.key_down(Keycode::Up);
        assert!(manager.is_key_pressed(Keycode::Up));

        manager.key_up(Keycode::Up);
        assert!(!manager.is_key_pressed(Keycode::Up));
    }

    #[test]
    fn test_just_pressed() {
        let mut manager = InputManager::new();

        // First frame - key down
        manager.key_down(Keycode::X);
        assert!(manager.is_key_just_pressed(Keycode::X));

        // Second frame - key still down
        manager.update();
        assert!(!manager.is_key_just_pressed(Keycode::X));
        assert!(manager.is_key_pressed(Keycode::X));
    }

    #[test]
    fn test_car_input() {
        let mut manager = InputManager::new();
        manager.key_down(Keycode::Up);
        manager.key_down(Keycode::Left);

        let input = manager.get_car_input();
        assert_eq!(input.throttle, 1.0);
        assert_eq!(input.steering, -1.0);
        assert_eq!(input.brake, 0.0);
    }

    #[test]
    fn test_car_input_reset() {
        let mut input = CarInput::new();
        input.throttle = 1.0;
        input.steering = -0.5;

        input.reset();
        assert_eq!(input.throttle, 0.0);
        assert_eq!(input.steering, 0.0);
    }
}
