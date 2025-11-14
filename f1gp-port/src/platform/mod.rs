//! Platform module
//!
//! Platform-specific abstractions for graphics, input, and windowing.

pub mod graphics;

pub use graphics::{Color, Rect, Renderer, SdlRenderer};
