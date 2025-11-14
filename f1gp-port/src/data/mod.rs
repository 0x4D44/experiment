//! Data module
//!
//! Contains data structures and loaders for F1GP game files

pub mod track;
pub mod objects;
pub mod loader;
pub mod parser;

pub use track::*;
pub use objects::*;
pub use loader::*;
pub use parser::*;
