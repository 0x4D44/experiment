use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        use ::rand::Rng;
        let mut rng = ::rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..150.0);
        let lifetime_val = rng.gen_range(0.3..0.8);
        let size_val = rng.gen_range(2.0..5.0);

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
            color,
            lifetime: 0.0,
            max_lifetime: lifetime_val,
            size: size_val,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
        self.velocity.y += 300.0 * delta_time; // Gravity
        self.lifetime += delta_time;
    }

    pub fn is_dead(&self) -> bool {
        self.lifetime >= self.max_lifetime
    }

    pub fn draw(&self) {
        let alpha = 1.0 - (self.lifetime / self.max_lifetime);
        let color = Color::new(self.color.r, self.color.g, self.color.b, alpha);
        draw_circle(self.position.x, self.position.y, self.size, color);
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn emit_jump(&mut self, x: f32, y: f32) {
        for _ in 0..8 {
            self.particles.push(Particle::new(x, y, WHITE));
        }
    }

    pub fn emit_landing(&mut self, x: f32, y: f32) {
        for _ in 0..10 {
            let mut particle = Particle::new(x, y, BROWN);
            particle.velocity.y = particle.velocity.y.abs(); // Only downward
            self.particles.push(particle);
        }
    }

    pub fn emit_collect(&mut self, x: f32, y: f32, color: Color) {
        for _ in 0..15 {
            self.particles.push(Particle::new(x, y, color));
        }
    }

    pub fn emit_enemy_death(&mut self, x: f32, y: f32) {
        for _ in 0..20 {
            self.particles.push(Particle::new(x, y, RED));
        }
    }

    pub fn emit_damage(&mut self, x: f32, y: f32) {
        for _ in 0..12 {
            self.particles.push(Particle::new(x, y, RED));
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.update(delta_time);
        }

        self.particles.retain(|p| !p.is_dead());
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

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}
