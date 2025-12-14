//! Track rendering system
//!
//! Renders track geometry, racing lines, and trackside objects.

use crate::data::track::{RacingLine, SurfaceType, Track, TrackSection};
use crate::platform::{Color, Rect, Renderer};
use crate::render::Camera;
use anyhow::Result;
use glam::{Vec2, Vec3};

/// Track edge point for rendering
#[derive(Debug, Clone, Copy)]
struct EdgePoint {
    left: Vec2,
    center: Vec2,
    right: Vec2,
}

/// Track renderer for 2D visualization
pub struct TrackRenderer {
    /// Edge points for track surface rendering
    edge_points: Vec<EdgePoint>,

    /// Cached track sections for detail rendering
    sections: Vec<TrackSection>,

    /// Cached racing line points
    racing_line_points: Vec<Vec2>,

    /// Track bounds for camera fitting
    pub bounds: Rect,
}

impl TrackRenderer {
    /// Create a new track renderer from a track
    pub fn new(track: &Track) -> Self {
        let edge_points = Self::generate_edge_points(&track.sections);
        let racing_line_points = Self::generate_racing_line_points(&track.racing_line);
        let bounds = Self::calculate_bounds(&edge_points);

        Self {
            edge_points,
            sections: track.sections.clone(),
            racing_line_points,
            bounds,
        }
    }

    /// Generate edge points (left, center, right) for each track section
    fn generate_edge_points(sections: &[TrackSection]) -> Vec<EdgePoint> {
        if sections.is_empty() {
            return Vec::new();
        }

        let mut points = Vec::with_capacity(sections.len());

        for i in 0..sections.len() {
            let section = &sections[i];
            let next_section = &sections[(i + 1) % sections.len()];

            // Get center position (use x and z for top-down view, y is elevation)
            let center = Vec2::new(section.position.x, section.position.z);

            // Calculate direction to next section
            let next_center = Vec2::new(next_section.position.x, next_section.position.z);
            let direction = (next_center - center).normalize_or_zero();

            // Calculate perpendicular for track width
            let perpendicular = Vec2::new(-direction.y, direction.x);

            // Use track width (default to 12m if not set)
            let half_width = (section.width.max(12.0)) / 2.0;

            let left = center + perpendicular * half_width;
            let right = center - perpendicular * half_width;

            points.push(EdgePoint {
                left,
                center,
                right,
            });
        }

        points
    }

    /// Generate visual points from racing line
    fn generate_racing_line_points(_racing_line: &RacingLine) -> Vec<Vec2> {
        // TODO: Convert racing line segments to 2D points
        Vec::new()
    }

    /// Calculate bounding box for all track points
    fn calculate_bounds(edge_points: &[EdgePoint]) -> Rect {
        if edge_points.is_empty() {
            return Rect::new(0.0, 0.0, 1000.0, 1000.0);
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for point in edge_points {
            for p in [point.left, point.center, point.right] {
                min_x = min_x.min(p.x);
                min_y = min_y.min(p.y);
                max_x = max_x.max(p.x);
                max_y = max_y.max(p.y);
            }
        }

        let padding = 200.0;
        Rect::new(
            min_x - padding,
            min_y - padding,
            (max_x - min_x) + padding * 2.0,
            (max_y - min_y) + padding * 2.0,
        )
    }

    /// Render the complete track
    pub fn render(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        // Render track surface (filled road)
        self.render_track_surface(renderer, camera)?;

        // Render track edges (white lines)
        self.render_track_edges(renderer, camera)?;

        // Render centerline
        self.render_centerline(renderer, camera)?;

        // Render kerbs
        self.render_kerbs(renderer, camera)?;

        // Render start/finish
        self.render_start_finish(renderer, camera)?;

        Ok(())
    }

    /// Render track surface as filled quads
    fn render_track_surface(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        if self.edge_points.len() < 2 {
            return Ok(());
        }

        // Draw track surface as thick lines between consecutive sections
        // (Since we don't have polygon fill, we'll draw multiple lines to create width)
        let asphalt_color = Color::rgb(60, 60, 70); // Dark asphalt

        for i in 0..self.edge_points.len() {
            let next_i = (i + 1) % self.edge_points.len();
            let p1 = &self.edge_points[i];
            let p2 = &self.edge_points[next_i];

            // Check visibility
            if !camera.is_visible(Vec3::new(p1.center.x, 0.0, p1.center.y), 200.0)
                && !camera.is_visible(Vec3::new(p2.center.x, 0.0, p2.center.y), 200.0)
            {
                continue;
            }

            // Convert to screen coordinates
            let s1_left = camera.world_to_screen(Vec3::new(p1.left.x, 0.0, p1.left.y));
            let s1_right = camera.world_to_screen(Vec3::new(p1.right.x, 0.0, p1.right.y));
            let s2_left = camera.world_to_screen(Vec3::new(p2.left.x, 0.0, p2.left.y));
            let s2_right = camera.world_to_screen(Vec3::new(p2.right.x, 0.0, p2.right.y));

            // Draw filled quad by drawing many horizontal lines
            // This is a simple scanline fill approximation
            let steps = 20;
            for t in 0..=steps {
                let frac = t as f32 / steps as f32;
                let start = s1_left.lerp(s1_right, frac);
                let end = s2_left.lerp(s2_right, frac);
                renderer.draw_line(start, end, asphalt_color)?;
            }

            // Also draw across for better coverage
            for t in 0..=steps {
                let frac = t as f32 / steps as f32;
                let left_interp = s1_left.lerp(s2_left, frac);
                let right_interp = s1_right.lerp(s2_right, frac);
                renderer.draw_line(left_interp, right_interp, asphalt_color)?;
            }
        }

        Ok(())
    }

    /// Render track edges (white lines)
    fn render_track_edges(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        if self.edge_points.len() < 2 {
            return Ok(());
        }

        let edge_color = Color::WHITE;

        for i in 0..self.edge_points.len() {
            let next_i = (i + 1) % self.edge_points.len();
            let p1 = &self.edge_points[i];
            let p2 = &self.edge_points[next_i];

            // Check visibility
            if !camera.is_visible(Vec3::new(p1.center.x, 0.0, p1.center.y), 200.0)
                && !camera.is_visible(Vec3::new(p2.center.x, 0.0, p2.center.y), 200.0)
            {
                continue;
            }

            // Left edge
            let s1_left = camera.world_to_screen(Vec3::new(p1.left.x, 0.0, p1.left.y));
            let s2_left = camera.world_to_screen(Vec3::new(p2.left.x, 0.0, p2.left.y));
            renderer.draw_line(s1_left, s2_left, edge_color)?;

            // Right edge
            let s1_right = camera.world_to_screen(Vec3::new(p1.right.x, 0.0, p1.right.y));
            let s2_right = camera.world_to_screen(Vec3::new(p2.right.x, 0.0, p2.right.y));
            renderer.draw_line(s1_right, s2_right, edge_color)?;
        }

        Ok(())
    }

    /// Render centerline (dashed yellow)
    fn render_centerline(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        if self.edge_points.len() < 2 {
            return Ok(());
        }

        let center_color = Color::rgb(200, 200, 0); // Yellow

        for i in 0..self.edge_points.len() {
            // Only draw every other segment for dashed effect
            if i % 2 != 0 {
                continue;
            }

            let next_i = (i + 1) % self.edge_points.len();
            let p1 = &self.edge_points[i];
            let p2 = &self.edge_points[next_i];

            // Check visibility
            if !camera.is_visible(Vec3::new(p1.center.x, 0.0, p1.center.y), 200.0)
                && !camera.is_visible(Vec3::new(p2.center.x, 0.0, p2.center.y), 200.0)
            {
                continue;
            }

            let s1 = camera.world_to_screen(Vec3::new(p1.center.x, 0.0, p1.center.y));
            let s2 = camera.world_to_screen(Vec3::new(p2.center.x, 0.0, p2.center.y));
            renderer.draw_line(s1, s2, center_color)?;
        }

        Ok(())
    }

    /// Render kerbs (red and white stripes)
    fn render_kerbs(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        if self.sections.is_empty() || self.edge_points.len() != self.sections.len() {
            return Ok(());
        }

        let kerb_red = Color::RED;
        let kerb_white = Color::WHITE;

        for (i, section) in self.sections.iter().enumerate() {
            let point = &self.edge_points[i];

            // Check visibility
            if !camera.is_visible(Vec3::new(point.center.x, 0.0, point.center.y), 200.0) {
                continue;
            }

            // Alternate red/white for kerb effect
            let color = if i % 2 == 0 { kerb_red } else { kerb_white };

            // Draw left kerb if present
            if section.has_left_kerb {
                let screen_pos = camera.world_to_screen(Vec3::new(point.left.x, 0.0, point.left.y));
                renderer.draw_filled_circle(screen_pos, 3.0, color)?;
            }

            // Draw right kerb if present
            if section.has_right_kerb {
                let screen_pos =
                    camera.world_to_screen(Vec3::new(point.right.x, 0.0, point.right.y));
                renderer.draw_filled_circle(screen_pos, 3.0, color)?;
            }
        }

        Ok(())
    }

    /// Render start/finish line
    fn render_start_finish(&self, renderer: &mut impl Renderer, camera: &Camera) -> Result<()> {
        if self.edge_points.is_empty() {
            return Ok(());
        }

        let start = &self.edge_points[0];

        // Check visibility
        if !camera.is_visible(Vec3::new(start.center.x, 0.0, start.center.y), 200.0) {
            return Ok(());
        }

        // Draw checkered pattern at start/finish
        let s_left = camera.world_to_screen(Vec3::new(start.left.x, 0.0, start.left.y));
        let s_right = camera.world_to_screen(Vec3::new(start.right.x, 0.0, start.right.y));

        // Draw start line
        renderer.draw_line(s_left, s_right, Color::WHITE)?;

        // Draw indicator circles
        renderer.draw_filled_circle(s_left, 5.0, Color::RED)?;
        renderer.draw_filled_circle(s_right, 5.0, Color::RED)?;

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
            let screen_pos = camera.world_to_screen(Vec3::new(point.x, 0.0, point.y));
            renderer.draw_filled_circle(screen_pos, 8.0, Color::YELLOW)?;
        }

        Ok(())
    }

    /// Get track points (for backward compatibility)
    pub fn track_points(&self) -> Vec<Vec2> {
        self.edge_points.iter().map(|p| p.center).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::track::*;

    fn create_test_track() -> Track {
        Track {
            name: "Test Track".to_string(),
            length: 1000.0,
            object_shapes: vec![],
            sections: vec![
                TrackSection {
                    position: Vec3::new(0.0, 0.0, 0.0),
                    width: 12.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
                TrackSection {
                    position: Vec3::new(100.0, 0.0, 0.0),
                    width: 12.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
                TrackSection {
                    position: Vec3::new(100.0, 0.0, 100.0),
                    width: 12.0,
                    surface: SurfaceType::Track,
                    length: 100.0,
                    ..TrackSection::default()
                },
                TrackSection {
                    position: Vec3::new(0.0, 0.0, 100.0),
                    width: 12.0,
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
        assert_eq!(renderer.edge_points.len(), 4);
    }

    #[test]
    fn test_edge_points_have_width() {
        let track = create_test_track();
        let renderer = TrackRenderer::new(&track);

        // Each edge point should have left and right different from center
        for point in &renderer.edge_points {
            let left_dist = (point.left - point.center).length();
            let right_dist = (point.right - point.center).length();
            assert!(left_dist > 0.0, "Left should be offset from center");
            assert!(right_dist > 0.0, "Right should be offset from center");
        }
    }

    #[test]
    fn test_calculate_bounds() {
        let track = create_test_track();
        let renderer = TrackRenderer::new(&track);
        assert!(renderer.bounds.width > 200.0); // Should include padding
        assert!(renderer.bounds.height > 200.0);
    }
}
