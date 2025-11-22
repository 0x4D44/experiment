use macroquad::prelude::*;

/// Particle for visual effects
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: f32,
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, size: f32, color: Color, lifetime: f32) -> Self {
        Self {
            position,
            velocity,
            size,
            color,
            lifetime,
            max_lifetime: lifetime,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;
        self.velocity *= 0.98; // Friction
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn alpha(&self) -> f32 {
        self.lifetime / self.max_lifetime
    }
}

/// Particle system manager
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
        self.particles.retain(|p| p.is_alive());
    }

    /// Create explosion particles
    pub fn create_explosion(&mut self, position: Vec2, count: usize, color: Color) {
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(50.0, 300.0);
            let velocity = vec2(angle.cos() * speed, angle.sin() * speed);
            let size = rand::gen_range(2.0, 8.0);
            let lifetime = rand::gen_range(0.3, 0.8);

            self.particles.push(Particle::new(
                position,
                velocity,
                size,
                color,
                lifetime,
            ));
        }
    }

    /// Create engine trail particles
    pub fn create_trail(&mut self, position: Vec2, color: Color) {
        let offset_x = rand::gen_range(-5.0, 5.0);
        let offset_y = rand::gen_range(0.0, 10.0);
        let velocity = vec2(offset_x * 10.0, 100.0 + offset_y * 10.0);

        self.particles.push(Particle::new(
            position,
            velocity,
            rand::gen_range(2.0, 4.0),
            color,
            rand::gen_range(0.2, 0.5),
        ));
    }

    /// Create bullet impact particles
    pub fn create_impact(&mut self, position: Vec2, count: usize) {
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(100.0, 200.0);
            let velocity = vec2(angle.cos() * speed, angle.sin() * speed);

            self.particles.push(Particle::new(
                position,
                velocity,
                rand::gen_range(1.0, 3.0),
                YELLOW,
                rand::gen_range(0.1, 0.3),
            ));
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
