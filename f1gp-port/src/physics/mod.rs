//! Physics module
//!
//! Provides physics simulation for cars and the game world.

pub mod car;
pub mod collision;
pub mod engine;

pub use car::{CarPhysics, TireGrip};
pub use collision::{CollisionResult, SurfacePhysics, TrackCollision};
pub use engine::{BodyId, PhysicsBody, PhysicsWorld, PHYSICS_TIMESTEP};
