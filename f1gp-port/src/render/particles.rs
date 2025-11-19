//! Particle system for visual effects
//!
//! Provides rain and other particle effects for enhanced visuals.

use crate::game::weather::WeatherCondition;
use crate::platform::{Color, Renderer};
use anyhow::Result;
use glam::Vec2;

/// A single rain particle
#[derive(Debug, Clone)]
struct Particle {
    /// Screen position
    position: Vec2,

    /// Velocity (pixels per second)
    velocity: Vec2,

    /// Visual length of the rain streak
    length: f32,

    /// Opacity (0.0 - 1.0)
    alpha: f32,
}

impl Particle {
    /// Create a new rain particle
    fn new_rain(screen_width: f32, _screen_height: f32, intensity: f32) -> Self {
        let x = fastrand::f32() * screen_width;
        let y = -10.0; // Start above screen

        // Rain falls mostly straight down, with slight wind
        let fall_speed = 400.0 + (fastrand::f32() * 200.0 * intensity);
        let wind = (fastrand::f32() - 0.5) * 50.0;

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(wind, fall_speed),
            length: 10.0 + (fastrand::f32() * 10.0 * intensity),
            alpha: 0.3 + (fastrand::f32() * 0.4),
        }
    }

    /// Update particle position
    fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }

    /// Check if particle is off-screen
    fn is_offscreen(&self, screen_height: f32) -> bool {
        self.position.y > screen_height + 10.0
    }
}

/// Particle system for visual effects
pub struct ParticleSystem {
    /// Active particles
    particles: Vec<Particle>,

    /// Maximum number of particles
    max_particles: usize,

    /// Spawn accumulator for rate limiting
    spawn_accumulator: f32,

    /// Screen dimensions
    screen_width: f32,
    screen_height: f32,
}

impl ParticleSystem {
    /// Create a new particle system
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            particles: Vec::new(),
            max_particles: 200,
            spawn_accumulator: 0.0,
            screen_width: screen_width as f32,
            screen_height: screen_height as f32,
        }
    }

    /// Update particles based on weather
    pub fn update(&mut self, delta_time: f32, weather: WeatherCondition) {
        // Determine spawn rate and intensity based on weather
        let (spawn_rate, intensity) = match weather {
            WeatherCondition::Dry => (0.0, 0.0),
            WeatherCondition::LightRain => (50.0, 0.5),  // 50 particles/second
            WeatherCondition::HeavyRain => (150.0, 1.0), // 150 particles/second
        };

        // Spawn new particles
        if spawn_rate > 0.0 {
            self.spawn_accumulator += delta_time * spawn_rate;

            while self.spawn_accumulator >= 1.0 && self.particles.len() < self.max_particles {
                self.particles.push(Particle::new_rain(
                    self.screen_width,
                    self.screen_height,
                    intensity,
                ));
                self.spawn_accumulator -= 1.0;
            }
        }

        // Update existing particles
        for particle in &mut self.particles {
            particle.update(delta_time);
        }

        // Remove off-screen particles
        self.particles.retain(|p| !p.is_offscreen(self.screen_height));
    }

    /// Render all particles
    pub fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        for particle in &self.particles {
            // Calculate end point of rain streak
            let start = particle.position;
            let end = start - Vec2::new(0.0, particle.length);

            // Rain color (light blue-white)
            let alpha = (particle.alpha * 255.0) as u8;
            let color = Color::rgba(200, 220, 255, alpha);

            renderer.draw_line(start, end, color)?;
        }

        Ok(())
    }

    /// Resize particle system
    pub fn resize(&mut self, width: u32, height: u32) {
        self.screen_width = width as f32;
        self.screen_height = height as f32;

        // Clear particles on resize to avoid visual glitches
        self.particles.clear();
    }

    /// Get current particle count
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        let particle = Particle::new_rain(800.0, 600.0, 1.0);
        assert!(particle.position.x >= 0.0 && particle.position.x <= 800.0);
        assert_eq!(particle.position.y, -10.0);
        assert!(particle.velocity.y > 0.0); // Falls down
    }

    #[test]
    fn test_particle_update() {
        let mut particle = Particle::new_rain(800.0, 600.0, 1.0);
        let initial_y = particle.position.y;

        particle.update(1.0); // 1 second

        assert!(particle.position.y > initial_y); // Moved down
    }

    #[test]
    fn test_particle_offscreen() {
        let mut particle = Particle::new_rain(800.0, 600.0, 1.0);

        assert!(!particle.is_offscreen(600.0));

        particle.position.y = 700.0;
        assert!(particle.is_offscreen(600.0));
    }

    #[test]
    fn test_particle_system_creation() {
        let system = ParticleSystem::new(800, 600);
        assert_eq!(system.particle_count(), 0);
        assert_eq!(system.screen_width, 800.0);
        assert_eq!(system.screen_height, 600.0);
    }

    #[test]
    fn test_particle_system_dry_weather() {
        let mut system = ParticleSystem::new(800, 600);

        // Update with dry weather - no particles should spawn
        system.update(1.0, WeatherCondition::Dry);

        assert_eq!(system.particle_count(), 0);
    }

    #[test]
    fn test_particle_system_rain() {
        let mut system = ParticleSystem::new(800, 600);

        // Update with light rain - particles should spawn
        system.update(1.0, WeatherCondition::LightRain);

        assert!(system.particle_count() > 0);
        assert!(system.particle_count() <= 50); // ~50 per second
    }

    #[test]
    fn test_particle_system_heavy_rain() {
        let mut system = ParticleSystem::new(800, 600);

        // Update with heavy rain - more particles
        system.update(1.0, WeatherCondition::HeavyRain);

        assert!(system.particle_count() > 50);
        assert!(system.particle_count() <= 150); // ~150 per second
    }

    #[test]
    fn test_particle_system_max_particles() {
        let mut system = ParticleSystem::new(800, 600);

        // Spawn lots of particles
        for _ in 0..10 {
            system.update(0.1, WeatherCondition::HeavyRain);
        }

        // Should not exceed max
        assert!(system.particle_count() <= system.max_particles);
    }

    #[test]
    fn test_particle_system_resize() {
        let mut system = ParticleSystem::new(800, 600);

        system.update(1.0, WeatherCondition::HeavyRain);
        assert!(system.particle_count() > 0);

        system.resize(1024, 768);

        assert_eq!(system.screen_width, 1024.0);
        assert_eq!(system.screen_height, 768.0);
        assert_eq!(system.particle_count(), 0); // Cleared on resize
    }
}
