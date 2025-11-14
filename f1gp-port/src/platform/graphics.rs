//! Graphics abstraction layer
//!
//! Provides platform-independent rendering interface with SDL2 implementation.

use anyhow::Result;
use glam::Vec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

/// RGBA color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);
    pub const GRAY: Self = Self::rgb(128, 128, 128);
    pub const DARK_GRAY: Self = Self::rgb(64, 64, 64);
    pub const LIGHT_GRAY: Self = Self::rgb(192, 192, 192);
}

impl From<Color> for SdlColor {
    fn from(color: Color) -> Self {
        SdlColor::RGBA(color.r, color.g, color.b, color.a)
    }
}

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

impl From<Rect> for SdlRect {
    fn from(rect: Rect) -> Self {
        SdlRect::new(
            rect.x as i32,
            rect.y as i32,
            rect.width as u32,
            rect.height as u32,
        )
    }
}

/// Renderer trait for platform-independent rendering
pub trait Renderer {
    /// Clear the screen with a color
    fn clear(&mut self, color: Color);

    /// Present the rendered frame
    fn present(&mut self);

    /// Draw a line
    fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color) -> Result<()>;

    /// Draw a rectangle outline
    fn draw_rect(&mut self, rect: Rect, color: Color) -> Result<()>;

    /// Draw a filled rectangle
    fn draw_filled_rect(&mut self, rect: Rect, color: Color) -> Result<()>;

    /// Draw a circle outline
    fn draw_circle(&mut self, center: Vec2, radius: f32, color: Color) -> Result<()>;

    /// Draw a filled circle
    fn draw_filled_circle(&mut self, center: Vec2, radius: f32, color: Color) -> Result<()>;

    /// Get viewport size
    fn viewport_size(&self) -> (u32, u32);
}

/// SDL2-based renderer implementation
pub struct SdlRenderer {
    canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl SdlRenderer {
    /// Create a new SDL2 renderer with the specified window dimensions
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let sdl_context = sdl2::init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize SDL2: {}", e))?;
        let video_subsystem = sdl_context
            .video()
            .map_err(|e| anyhow::anyhow!("Failed to initialize video subsystem: {}", e))?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create window: {}", e))?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create canvas: {}", e))?;

        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| anyhow::anyhow!("Failed to create event pump: {}", e))?;

        Ok(Self { canvas, event_pump })
    }

    /// Poll for events and return true if should continue running
    pub fn poll_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                _ => {}
            }
        }
        true
    }

    /// Delay for frame timing
    pub fn delay(&self, duration: Duration) {
        std::thread::sleep(duration);
    }
}

impl Renderer for SdlRenderer {
    fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(SdlColor::from(color));
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color) -> Result<()> {
        self.canvas.set_draw_color(SdlColor::from(color));
        self.canvas
            .draw_line(
                (start.x as i32, start.y as i32),
                (end.x as i32, end.y as i32),
            )
            .map_err(|e| anyhow::anyhow!("Failed to draw line: {}", e))?;
        Ok(())
    }

    fn draw_rect(&mut self, rect: Rect, color: Color) -> Result<()> {
        self.canvas.set_draw_color(SdlColor::from(color));
        self.canvas
            .draw_rect(SdlRect::from(rect))
            .map_err(|e| anyhow::anyhow!("Failed to draw rectangle: {}", e))?;
        Ok(())
    }

    fn draw_filled_rect(&mut self, rect: Rect, color: Color) -> Result<()> {
        self.canvas.set_draw_color(SdlColor::from(color));
        self.canvas
            .fill_rect(SdlRect::from(rect))
            .map_err(|e| anyhow::anyhow!("Failed to draw filled rectangle: {}", e))?;
        Ok(())
    }

    fn draw_circle(&mut self, center: Vec2, radius: f32, color: Color) -> Result<()> {
        self.canvas.set_draw_color(SdlColor::from(color));
        draw_circle_outline(&mut self.canvas, center, radius)?;
        Ok(())
    }

    fn draw_filled_circle(&mut self, center: Vec2, radius: f32, color: Color) -> Result<()> {
        self.canvas.set_draw_color(SdlColor::from(color));
        draw_circle_filled(&mut self.canvas, center, radius)?;
        Ok(())
    }

    fn viewport_size(&self) -> (u32, u32) {
        self.canvas.output_size().unwrap_or((800, 600))
    }
}

/// Draw circle outline using midpoint circle algorithm
fn draw_circle_outline(canvas: &mut Canvas<Window>, center: Vec2, radius: f32) -> Result<()> {
    let cx = center.x as i32;
    let cy = center.y as i32;
    let r = radius as i32;

    let mut x = 0;
    let mut y = r;
    let mut d = 3 - 2 * r;

    while x <= y {
        // Draw 8 symmetric points
        canvas.draw_point((cx + x, cy + y)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_point((cx - x, cy + y)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_point((cx + x, cy - y)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_point((cx - x, cy - y)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_point((cx + y, cy + x)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_point((cx - y, cy + x)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_point((cx + y, cy - x)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_point((cx - y, cy - x)).map_err(|e| anyhow::anyhow!("{}", e))?;

        if d < 0 {
            d = d + 4 * x + 6;
        } else {
            d = d + 4 * (x - y) + 10;
            y -= 1;
        }
        x += 1;
    }

    Ok(())
}

/// Draw filled circle using horizontal lines
fn draw_circle_filled(canvas: &mut Canvas<Window>, center: Vec2, radius: f32) -> Result<()> {
    let cx = center.x as i32;
    let cy = center.y as i32;
    let r = radius as i32;

    let mut x = 0;
    let mut y = r;
    let mut d = 3 - 2 * r;

    while x <= y {
        // Draw horizontal lines for filled circle
        canvas.draw_line((cx - x, cy + y), (cx + x, cy + y)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_line((cx - x, cy - y), (cx + x, cy - y)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_line((cx - y, cy + x), (cx + y, cy + x)).map_err(|e| anyhow::anyhow!("{}", e))?;
        canvas.draw_line((cx - y, cy - x), (cx + y, cy - x)).map_err(|e| anyhow::anyhow!("{}", e))?;

        if d < 0 {
            d = d + 4 * x + 6;
        } else {
            d = d + 4 * (x - y) + 10;
            y -= 1;
        }
        x += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::BLACK, Color::rgb(0, 0, 0));
        assert_eq!(Color::WHITE, Color::rgb(255, 255, 255));
        assert_eq!(Color::RED, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_rect_center() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        let center = rect.center();
        assert_eq!(center.x, 60.0);
        assert_eq!(center.y, 45.0);
    }
}
