//! Car rendering system
//!
//! Renders cars on track with liveries and visual effects.

use crate::data::car::CarSpec;
use crate::platform::{Color, Rect, Renderer};
use crate::render::{Camera, SpriteAtlas, SpriteSheet};
use anyhow::Result;
use glam::{Vec2, Vec3};

/// Car state for rendering
#[derive(Debug, Clone)]
pub struct CarState {
    /// Car position in world space
    pub position: Vec3,

    /// Car rotation in radians (0 = facing right/east)
    pub rotation: f32,

    /// Car velocity (m/s)
    pub velocity: Vec2,

    /// Car specification
    pub spec: CarSpec,

    /// Current driver name
    pub driver_name: String,
}

/// Car renderer for 2D visualization
pub struct CarRenderer {
    /// Car dimensions for rendering (meters to pixels scale)
    car_length: f32,
    car_width: f32,
    sprite: Option<SpriteResources>,
}

#[derive(Debug, Clone)]
struct SpriteResources {
    atlas: SpriteAtlas,
    sheet: SpriteSheet,
}

impl CarRenderer {
    /// Create a new car renderer
    pub fn new() -> Self {
        Self {
            car_length: 4.5, // Approximate F1 car length in meters
            car_width: 2.0,  // Approximate F1 car width in meters
            sprite: None,
        }
    }

    /// Create a renderer with a sprite atlas already loaded
    pub fn with_sprite_atlas(atlas: SpriteAtlas, sheet: SpriteSheet) -> Self {
        let mut renderer = Self::new();
        renderer.sprite = Some(SpriteResources { atlas, sheet });
        renderer
    }

    /// Set/replace the sprite atlas at runtime
    pub fn set_sprite_resources(&mut self, atlas: SpriteAtlas, sheet: SpriteSheet) {
        self.sprite = Some(SpriteResources { atlas, sheet });
    }

    /// Render a single car
    pub fn render_car(
        &self,
        renderer: &mut impl Renderer,
        car: &CarState,
        camera: &Camera,
    ) -> Result<()> {
        // Skip if car is not visible
        if !camera.is_visible(car.position, 50.0) {
            return Ok(());
        }

        let screen_pos = camera.world_to_screen(car.position);

        // Calculate car dimensions in screen space
        let _length = self.car_length * camera.zoom; // Reserved for future sprite rendering
        let width = self.car_width * camera.zoom;

        if let Some(resources) = &self.sprite {
            if let Some(frame) = resources.atlas.frame("car_body") {
                let src = Rect::new(
                    frame.x as f32,
                    frame.y as f32,
                    frame.width as f32,
                    frame.height as f32,
                );
                let scale = camera.zoom.max(0.1);
                let dst_width = frame.width as f32 * scale;
                let dst_height = frame.height as f32 * scale;
                let dst = Rect::new(
                    screen_pos.x - dst_width / 2.0,
                    screen_pos.y - dst_height / 2.0,
                    dst_width,
                    dst_height,
                );
                renderer.draw_rgba_region(
                    resources.sheet.cache_key(),
                    resources.sheet.pixels(),
                    resources.sheet.width(),
                    resources.sheet.height(),
                    src,
                    dst,
                    car.rotation.to_degrees(),
                )?;
                return Ok(());
            }
        }

        // Get team colors from livery
        let primary_color = if !car.spec.livery_colors.is_empty() {
            let (r, g, b) = car.spec.livery_colors[0];
            Color::rgb(r, g, b)
        } else {
            Color::GRAY
        };

        // Draw car as a filled circle (simplified for now)
        renderer.draw_filled_circle(screen_pos, width.max(3.0), primary_color)?;

        // Draw direction indicator
        let dir_offset = Vec2::new(car.rotation.cos(), car.rotation.sin()) * width * 1.5;
        let dir_pos = screen_pos + dir_offset;
        renderer.draw_line(screen_pos, dir_pos, Color::WHITE)?;

        Ok(())
    }

    /// Render multiple cars
    pub fn render_cars(
        &self,
        renderer: &mut impl Renderer,
        cars: &[CarState],
        camera: &Camera,
    ) -> Result<()> {
        for car in cars {
            self.render_car(renderer, car, camera)?;
        }
        Ok(())
    }

    /// Render car with driver name label
    pub fn render_car_with_label(
        &self,
        renderer: &mut impl Renderer,
        car: &CarState,
        camera: &Camera,
    ) -> Result<()> {
        // Render the car
        self.render_car(renderer, car, camera)?;

        // Draw a simple indicator for the driver name
        // (Text rendering would require SDL2_ttf)
        let screen_pos = camera.world_to_screen(car.position);
        let label_offset = Vec2::new(0.0, -15.0);
        let label_pos = screen_pos + label_offset;

        // Draw a small rectangle as a placeholder for driver name
        renderer.draw_filled_rect(
            Rect::new(label_pos.x - 10.0, label_pos.y, 20.0, 3.0),
            Color::WHITE,
        )?;

        Ok(())
    }

    /// Render car with visual effects (tire smoke, sparks, etc.)
    pub fn render_car_with_effects(
        &self,
        renderer: &mut impl Renderer,
        car: &CarState,
        camera: &Camera,
        show_tire_smoke: bool,
        show_sparks: bool,
    ) -> Result<()> {
        // Render the car
        self.render_car(renderer, car, camera)?;

        let screen_pos = camera.world_to_screen(car.position);

        // Tire smoke effect when sliding
        if show_tire_smoke {
            let smoke_offset = Vec2::new(-car.rotation.cos(), -car.rotation.sin()) * 8.0;
            let smoke_pos = screen_pos + smoke_offset;
            renderer.draw_filled_circle(smoke_pos, 4.0, Color::LIGHT_GRAY)?;
        }

        // Sparks effect when bottoming out
        if show_sparks {
            for i in 0..3 {
                let angle = car.rotation + (i as f32 - 1.0) * 0.3;
                let spark_offset = Vec2::new(angle.cos(), angle.sin()) * 6.0;
                let spark_pos = screen_pos + spark_offset;
                renderer.draw_filled_circle(spark_pos, 1.0, Color::YELLOW)?;
            }
        }

        Ok(())
    }
}

impl Default for CarRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::car::{AeroSpec, CarDimensions, CarSpec, EngineSpec};

    fn create_test_car() -> CarState {
        CarState {
            position: Vec3::new(100.0, 100.0, 0.0),
            rotation: 0.0,
            velocity: Vec2::new(50.0, 0.0),
            spec: CarSpec {
                name: "Test Car".to_string(),
                team: "Test Team".to_string(),
                engine: EngineSpec {
                    power_curve: vec![],
                    max_rpm: 15000.0,
                    torque_curve: vec![],
                    response: 0.8,
                },
                aerodynamics: AeroSpec {
                    downforce: 2.0,
                    drag: 0.8,
                    front_wing: 10.0,
                    rear_wing: 15.0,
                },
                mass: 500.0,
                dimensions: CarDimensions {
                    length: 4.0,
                    width: 1.8,
                    height: 0.9,
                    wheelbase: 2.5,
                },
                livery_colors: vec![(255, 0, 0), (255, 255, 255)],
            },
            driver_name: "Test Driver".to_string(),
        }
    }

    #[test]
    fn test_car_renderer_creation() {
        let renderer = CarRenderer::new();
        assert_eq!(renderer.car_length, 4.5);
        assert_eq!(renderer.car_width, 2.0);
    }

    #[test]
    fn test_car_state_creation() {
        let car = create_test_car();
        assert_eq!(car.position.x, 100.0);
        assert_eq!(car.rotation, 0.0);
        assert_eq!(car.driver_name, "Test Driver");
    }
}
