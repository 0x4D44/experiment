//! HUD (Heads-Up Display) rendering
//!
//! Renders race information overlay including lap times, speed, gear, RPM, etc.

use crate::platform::{Color, Rect, Renderer};
use anyhow::Result;
use glam::Vec2;

/// HUD display for race information
pub struct Hud {
    /// Screen dimensions
    screen_width: u32,
    screen_height: u32,
}

/// Race telemetry data to display
pub struct Telemetry {
    /// Current speed (km/h)
    pub speed: f32,
    /// Current gear
    pub gear: i8,
    /// Engine RPM
    pub rpm: f32,
    /// Current lap number
    pub current_lap: u32,
    /// Current lap time (seconds)
    pub current_lap_time: f32,
    /// Best lap time (seconds), if available
    pub best_lap_time: Option<f32>,
    /// Delta to best lap (seconds), if available
    pub delta_time: Option<f32>,
    /// Is car on track?
    pub on_track: bool,
}

impl Hud {
    /// Create new HUD
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            screen_width,
            screen_height,
        }
    }

    /// Update screen dimensions
    pub fn resize(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
    }

    /// Render HUD overlay
    pub fn render(&self, renderer: &mut dyn Renderer, telemetry: &Telemetry) -> Result<()> {
        // Draw telemetry panel in bottom-left
        self.draw_telemetry_panel(renderer, telemetry)?;

        // Draw lap timing in top-right
        self.draw_timing_panel(renderer, telemetry)?;

        // Draw gear indicator in bottom-right
        self.draw_gear_indicator(renderer, telemetry)?;

        // Draw status indicators
        self.draw_status_indicators(renderer, telemetry)?;

        Ok(())
    }

    /// Draw telemetry panel (speed, RPM)
    fn draw_telemetry_panel(&self, renderer: &mut dyn Renderer, telemetry: &Telemetry) -> Result<()> {
        let panel_x = 10.0;
        let panel_y = self.screen_height as f32 - 120.0;
        let panel_width = 200.0;
        let panel_height = 110.0;

        // Semi-transparent background
        let bg_rect = Rect::new(panel_x, panel_y, panel_width, panel_height);
        renderer.draw_filled_rect(bg_rect, Color::rgba(0, 0, 0, 180))?;
        renderer.draw_rect(bg_rect, Color::rgb(255, 255, 255))?;

        // Speed display
        let speed_y = panel_y + 15.0;
        renderer.draw_text(
            &format!("SPEED: {:.0} km/h", telemetry.speed),
            Vec2::new(panel_x + 10.0, speed_y),
            20.0,
            Color::rgb(255, 255, 255),
        )?;

        // Speed bar
        let max_speed = 350.0; // km/h
        let speed_ratio = (telemetry.speed / max_speed).min(1.0);
        let bar_width = (panel_width - 20.0) * speed_ratio;
        let speed_bar = Rect::new(panel_x + 10.0, speed_y + 25.0, bar_width, 15.0);
        let speed_color = if telemetry.speed > 300.0 {
            Color::rgb(255, 0, 0) // Red for high speed
        } else if telemetry.speed > 200.0 {
            Color::rgb(255, 165, 0) // Orange for medium speed
        } else {
            Color::rgb(0, 255, 0) // Green for low speed
        };
        renderer.draw_filled_rect(speed_bar, speed_color)?;

        // RPM display
        let rpm_y = panel_y + 55.0;
        renderer.draw_text(
            &format!("RPM: {:.0}", telemetry.rpm),
            Vec2::new(panel_x + 10.0, rpm_y),
            20.0,
            Color::rgb(255, 255, 255),
        )?;

        // RPM bar
        let max_rpm = 15000.0;
        let rpm_ratio = (telemetry.rpm / max_rpm).min(1.0);
        let rpm_bar_width = (panel_width - 20.0) * rpm_ratio;
        let rpm_bar = Rect::new(panel_x + 10.0, rpm_y + 25.0, rpm_bar_width, 15.0);
        let rpm_color = if telemetry.rpm > 13000.0 {
            Color::rgb(255, 0, 0) // Red zone
        } else if telemetry.rpm > 10000.0 {
            Color::rgb(255, 255, 0) // Yellow
        } else {
            Color::rgb(0, 255, 0) // Green
        };
        renderer.draw_filled_rect(rpm_bar, rpm_color)?;

        Ok(())
    }

    /// Draw timing panel (lap times)
    fn draw_timing_panel(&self, renderer: &mut dyn Renderer, telemetry: &Telemetry) -> Result<()> {
        let panel_x = self.screen_width as f32 - 250.0;
        let panel_y = 10.0;
        let panel_width = 240.0;
        let panel_height = 110.0;

        // Semi-transparent background
        let bg_rect = Rect::new(panel_x, panel_y, panel_width, panel_height);
        renderer.draw_filled_rect(bg_rect, Color::rgba(0, 0, 0, 180))?;
        renderer.draw_rect(bg_rect, Color::rgb(255, 255, 255))?;

        // Current lap
        renderer.draw_text(
            &format!("LAP {}", telemetry.current_lap),
            Vec2::new(panel_x + 10.0, panel_y + 15.0),
            24.0,
            Color::rgb(255, 255, 0),
        )?;

        // Current lap time
        let time_str = format_time(telemetry.current_lap_time);
        renderer.draw_text(
            &format!("Current: {}", time_str),
            Vec2::new(panel_x + 10.0, panel_y + 45.0),
            20.0,
            Color::rgb(255, 255, 255),
        )?;

        // Best lap time
        if let Some(best) = telemetry.best_lap_time {
            let best_str = format_time(best);
            renderer.draw_text(
                &format!("Best: {}", best_str),
                Vec2::new(panel_x + 10.0, panel_y + 70.0),
                20.0,
                Color::rgb(0, 255, 0),
            )?;
        }

        // Delta time
        if let Some(delta) = telemetry.delta_time {
            let delta_str = if delta > 0.0 {
                format!("+{:.3}", delta)
            } else {
                format!("{:.3}", delta)
            };
            let delta_color = if delta > 0.0 {
                Color::rgb(255, 0, 0) // Slower (red)
            } else {
                Color::rgb(0, 255, 0) // Faster (green)
            };
            renderer.draw_text(
                &delta_str,
                Vec2::new(panel_x + 10.0, panel_y + 95.0),
                18.0,
                delta_color,
            )?;
        }

        Ok(())
    }

    /// Draw gear indicator
    fn draw_gear_indicator(&self, renderer: &mut dyn Renderer, telemetry: &Telemetry) -> Result<()> {
        let panel_x = self.screen_width as f32 - 100.0;
        let panel_y = self.screen_height as f32 - 100.0;
        let panel_size = 90.0;

        // Semi-transparent background
        let bg_rect = Rect::new(panel_x, panel_y, panel_size, panel_size);
        renderer.draw_filled_rect(bg_rect, Color::rgba(0, 0, 0, 180))?;
        renderer.draw_rect(bg_rect, Color::rgb(255, 255, 255))?;

        // Gear number
        let gear_str = if telemetry.gear == 0 {
            "R".to_string()
        } else {
            telemetry.gear.to_string()
        };

        renderer.draw_text(
            &gear_str,
            Vec2::new(panel_x + panel_size / 2.0 - 15.0, panel_y + 20.0),
            48.0,
            Color::rgb(255, 255, 0),
        )?;

        Ok(())
    }

    /// Draw status indicators (on/off track, etc.)
    fn draw_status_indicators(&self, renderer: &mut dyn Renderer, telemetry: &Telemetry) -> Result<()> {
        // Off-track warning
        if !telemetry.on_track {
            let warning_x = self.screen_width as f32 / 2.0 - 80.0;
            let warning_y = self.screen_height as f32 - 50.0;

            renderer.draw_filled_rect(
                Rect::new(warning_x, warning_y, 160.0, 35.0),
                Color::rgba(255, 0, 0, 200),
            )?;

            renderer.draw_text(
                "OFF TRACK!",
                Vec2::new(warning_x + 10.0, warning_y + 5.0),
                24.0,
                Color::rgb(255, 255, 255),
            )?;
        }

        Ok(())
    }
}

/// Format time in MM:SS.mmm format
fn format_time(seconds: f32) -> String {
    let minutes = (seconds / 60.0) as u32;
    let secs = seconds % 60.0;
    format!("{:02}:{:06.3}", minutes, secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hud_creation() {
        let hud = Hud::new(1280, 720);
        assert_eq!(hud.screen_width, 1280);
        assert_eq!(hud.screen_height, 720);
    }

    #[test]
    fn test_hud_resize() {
        let mut hud = Hud::new(800, 600);
        hud.resize(1920, 1080);
        assert_eq!(hud.screen_width, 1920);
        assert_eq!(hud.screen_height, 1080);
    }

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(0.0), "00:00.000");
        assert_eq!(format_time(65.123), "01:05.123");
        assert_eq!(format_time(125.456), "02:05.456");
    }

    #[test]
    fn test_telemetry_creation() {
        let telemetry = Telemetry {
            speed: 250.0,
            gear: 5,
            rpm: 12000.0,
            current_lap: 1,
            current_lap_time: 65.123,
            best_lap_time: Some(64.500),
            delta_time: Some(0.623),
            on_track: true,
        };
        assert_eq!(telemetry.speed, 250.0);
        assert_eq!(telemetry.gear, 5);
    }
}
