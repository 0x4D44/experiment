//! Data module
//!
//! Contains data structures and loaders for F1GP game files

pub mod asset;
pub mod car;
pub mod fixtures;
pub mod loader;
pub mod objects;
pub mod parser;
pub mod track;

pub use asset::*;
pub use car::*;
pub use fixtures::*;
pub use loader::*;
pub use objects::*;
pub use parser::*;
pub use track::*;
