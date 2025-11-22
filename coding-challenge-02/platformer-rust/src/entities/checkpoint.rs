use crate::physics::AABB;
use macroquad::prelude::*;

pub struct Checkpoint {
    pub position: Vec2,
    pub activated: bool,
    pub animation_timer: f32,
}

impl Checkpoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            activated: false,
            animation_timer: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.animation_timer += delta_time;
    }

    pub fn activate(&mut self) {
        self.activated = true;
    }

    pub fn aabb(&self) -> AABB {
        AABB::new(self.position.x - 16.0, self.position.y - 40.0, 32.0, 40.0)
    }

    pub fn draw(&self) {
        let x = self.position.x;
        let y = self.position.y;

        // Draw pole
        draw_rectangle(x - 2.0, y - 40.0, 4.0, 40.0, DARKGRAY);

        // Draw flag
        let flag_color = if self.activated { GREEN } else { GRAY };
        let wave = (self.animation_timer * 3.0).sin() * 3.0;

        draw_triangle(
            Vec2::new(x, y - 35.0),
            Vec2::new(x, y - 25.0),
            Vec2::new(x + 20.0 + wave, y - 30.0),
            flag_color,
        );

        // Draw activation glow
        if self.activated {
            let glow_size = 20.0 + (self.animation_timer * 2.0).sin() * 5.0;
            draw_circle(x, y - 30.0, glow_size, Color::new(0.0, 1.0, 0.0, 0.2));
        }
    }
}
