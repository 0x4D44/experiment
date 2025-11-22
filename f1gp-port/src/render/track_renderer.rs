//! Track rendering system
//!
//! Renders track geometry, racing lines, and trackside objects.

use crate::data::track::{RacingLine, Track, TrackSection};
use crate::platform::{Color, Rect, Renderer};
use crate::render::Camera;
use anyhow::Result;
use glam::Vec2;

/// Track renderer for 2D visualization
pub struct TrackRenderer {
    /// Cached rendering data for track surface
    track_points: Vec<Vec2>,

    /// Cached racing line points
    racing_line_points: Vec<Vec2>,

    /// Track bounds for camera fitting
    pub bounds: Rect,
}

impl TrackRenderer {
    /// Create a new track renderer from a track
    pub fn new(track: &Track) -> Self {
        let track_points = Self::generate_track_points(&track.sections);
        let racing_line_points = Self::generate_racing_line_points(&track.racing_line);
        let bounds = Self::calculate_bounds(&track_points);

        Self {
            track_points,
            racing_line_points,
            bounds,
        }
    }

    /// Generate visual points from track sections
    fn generate_track_points(sections: &[TrackSection]) -> Vec<Vec2> {
        sections
            .iter()
            .map(|section| Vec2::new(section.position.x, section.position.y))
            .collect()
    }

    /// Generate visual points from racing line
    fn generate_racing_line_points(_racing_line: &RacingLine) -> Vec<Vec2> {
        // TODO: Convert racing line segments to 3D points once coordinate calculation is implemented
        // For now, return empty - racing line will be calculated from track sections
        Vec::new()
    }

    /// Calculate bounding box for all track points
    fn calculate_bounds(points: &[Vec2]) -> Rect {
        if points.is_empty() {
            return Rect::new(0.0, 0.0, 1000.0, 1000.0);
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for point in points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        let padding = 100.0;
        Rect::new(
            min_x - padding,
            min_y - padding,
            (max_x - min_x) + padding * 2.0,
            (max_y - min_y) + padding * 2.0,
        )
    }

    /// Render the complete track
    pub fn render(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        // Render track surface
        self.render_track_surface(renderer, camera)?;

        // Render racing line
        self.render_racing_line(renderer, camera)?;

        // Render track details
        self.render_track_details(renderer, camera)?;

        Ok(())
    }

    /// Render track surface
    fn render_track_surface(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        if self.track_points.len() < 2 {
            return Ok(());
        }

        // Draw track outline
        for i in 0..self.track_points.len() {
            let next_i = (i + 1) % self.track_points.len();
            let p1 = camera.world_to_screen(self.track_points[i].extend(0.0));
            let p2 = camera.world_to_screen(self.track_points[next_i].extend(0.0));

            // Only draw if visible
            if camera.is_visible(self.track_points[i].extend(0.0), 100.0)
                || camera.is_visible(self.track_points[next_i].extend(0.0), 100.0)
            {
                renderer.draw_line(p1, p2, Color::DARK_GRAY)?;
            }
        }

        Ok(())
    }

    /// Render racing line
    fn render_racing_line(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        if self.racing_line_points.len() < 2 {
            return Ok(());
        }

        // Draw racing line as connected segments
        for i in 0..self.racing_line_points.len() - 1 {
            let p1 = camera.world_to_screen(self.racing_line_points[i].extend(0.0));
            let p2 = camera.world_to_screen(self.racing_line_points[i + 1].extend(0.0));

            // Only draw if visible
            if camera.is_visible(self.racing_line_points[i].extend(0.0), 100.0)
                || camera.is_visible(self.racing_line_points[i + 1].extend(0.0), 100.0)
            {
                renderer.draw_line(p1, p2, Color::GREEN)?;
            }
        }

        // Close the loop if we have points
        if !self.racing_line_points.is_empty() {
            let p1 = camera.world_to_screen(
                self.racing_line_points[self.racing_line_points.len() - 1].extend(0.0),
            );
            let p2 = camera.world_to_screen(self.racing_line_points[0].extend(0.0));
            renderer.draw_line(p1, p2, Color::GREEN)?;
        }

        Ok(())
    }

    /// Render track details (kerbs, pit lane, etc.)
    fn render_track_details(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        // Draw start/finish line if we have track points
        if !self.track_points.is_empty() {
            let start_pos = camera.world_to_screen(self.track_points[0].extend(0.0));
            renderer.draw_filled_circle(start_pos, 5.0, Color::RED)?;
        }

        Ok(())
    }

    /// Render track with highlight
    pub fn render_with_highlight(
        &self,
        renderer: &mut impl Renderer,
        camera: &Camera,
        highlight_point: Option<Vec2>,
    ) -> Result<()> {
        // Render normal track
        self.render(renderer, camera)?;

        // Render highlight if provided
        if let Some(point) = highlight_point {
            let screen_pos = camera.world_to_screen(point.extend(0.0));
            renderer.draw_filled_circle(screen_pos, 8.0, Color::YELLOW)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::track::*;
    use glam::Vec3;

    fn create_test_track() -> Track {
        Track {
            name: "Test Track".to_string(),
            length: 1000.0,
            object_shapes: vec![],
            sections: vec![
                TrackSection {
                    position: Vec3::new(0.0, 0.0, 0.0),
                    width: 10.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
                TrackSection {
                    position: Vec3::new(100.0, 0.0, 0.0),
                    width: 10.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
                TrackSection {
                    position: Vec3::new(100.0, 100.0, 0.0),
                    width: 10.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
                TrackSection {
                    position: Vec3::new(0.0, 100.0, 0.0),
                    width: 10.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
            ],
            racing_line: RacingLine {
                displacement: 0,
                segments: Vec::new(),
            },
            ai_behavior: AIBehavior::default(),
            pit_lane: vec![],
            cameras: vec![],
            checksum: 0,
        }
    }

    #[test]
    fn test_track_renderer_creation() {
        let track = create_test_track();
        let renderer = TrackRenderer::new(&track);
        assert_eq!(renderer.track_points.len(), 4);
    }

    #[test]
    fn test_calculate_bounds() {
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(0.0, 100.0),
        ];
        let bounds = TrackRenderer::calculate_bounds(&points);
        assert!(bounds.width > 200.0); // Should include padding
        assert!(bounds.height > 200.0);
    }

    #[test]
    fn test_empty_track_points() {
        let points: Vec<Vec2> = vec![];
        let bounds = TrackRenderer::calculate_bounds(&points);
        assert_eq!(bounds.width, 1000.0); // Default size
    }
}
