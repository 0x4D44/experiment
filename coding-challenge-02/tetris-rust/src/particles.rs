/// Particle effects for visual polish
use macroquad::prelude::*;
use ::rand::{Rng, thread_rng};

#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
    pub size: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..150.0);

        Particle {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            life: rng.gen_range(0.3..0.8),
            max_life: rng.gen_range(0.3..0.8),
            color,
            size: rng.gen_range(3.0..8.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.vy += 300.0 * dt; // Gravity
        self.life -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    pub fn draw(&self) {
        let alpha = (self.life / self.max_life).clamp(0.0, 1.0);
        let mut color = self.color;
        color.a = alpha;
        draw_circle(self.x, self.y, self.size, color);
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        ParticleSystem {
            particles: Vec::new(),
        }
    }

    pub fn emit(&mut self, x: f32, y: f32, color: Color, count: usize) {
        for _ in 0..count {
            self.particles.push(Particle::new(x, y, color));
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update all particles
        for particle in &mut self.particles {
            particle.update(dt);
        }

        // Remove dead particles
        self.particles.retain(|p| p.is_alive());
    }

    pub fn draw(&self) {
        for particle in &self.particles {
            particle.draw();
        }
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }
}
