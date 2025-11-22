/// Input handling module
/// This module is intentionally simple as input is handled in main.rs
/// This file exists to support potential future input abstraction
#[allow(dead_code)]
pub struct InputState {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub space_pressed: bool,
    pub pause_pressed: bool,
}

impl InputState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            left_pressed: false,
            right_pressed: false,
            space_pressed: false,
            pause_pressed: false,
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.left_pressed = false;
        self.right_pressed = false;
        self.space_pressed = false;
        self.pause_pressed = false;
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
