//! Physics module
//!
//! Provides physics simulation for cars and the game world.

pub mod engine;
pub mod car;
pub mod collision;

pub use engine::{BodyId, PhysicsBody, PhysicsWorld, PHYSICS_TIMESTEP};
pub use car::{CarPhysics, TireGrip};
pub use collision::{CollisionResult, SurfacePhysics, TrackCollision};