use macroquad::prelude::*;

pub struct Camera {
    pub position: Vec2,
    pub target: Vec2,
    pub smoothness: f32,
    pub bounds: Option<(f32, f32, f32, f32)>, // (min_x, min_y, max_x, max_y)
    pub screen_width: f32,
    pub screen_height: f32,
}

impl Camera {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            target: Vec2::ZERO,
            smoothness: 8.0,
            bounds: None,
            screen_width,
            screen_height,
        }
    }

    pub fn set_bounds(&mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        self.bounds = Some((min_x, min_y, max_x, max_y));
    }

    pub fn follow(&mut self, target_x: f32, target_y: f32) {
        self.target = Vec2::new(
            target_x - self.screen_width / 2.0,
            target_y - self.screen_height / 2.0,
        );
    }

    pub fn update(&mut self, delta_time: f32) {
        // Smooth camera movement
        let direction = self.target - self.position;
        self.position += direction * self.smoothness * delta_time;

        // Apply bounds if set
        if let Some((min_x, min_y, max_x, max_y)) = self.bounds {
            self.position.x = self.position.x.clamp(min_x, max_x - self.screen_width);
            self.position.y = self.position.y.clamp(min_y, max_y - self.screen_height);
        }
    }

    pub fn apply(&self) {
        let cam = Camera2D {
            target: self.position + Vec2::new(self.screen_width / 2.0, self.screen_height / 2.0),
            zoom: Vec2::new(2.0 / self.screen_width, -2.0 / self.screen_height),
            offset: Vec2::ZERO,
            rotation: 0.0,
            render_target: None,
            viewport: None,
        };
        set_camera(&cam);
    }

    pub fn reset(&self) {
        set_default_camera();
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        world_pos - self.position
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        screen_pos + self.position
    }
}
