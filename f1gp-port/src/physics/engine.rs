//! Physics engine core
//!
//! Provides the main physics simulation engine with fixed timestep integration.

use glam::{Quat, Vec3};

/// Fixed timestep for physics simulation (60 Hz)
pub const PHYSICS_TIMESTEP: f32 = 1.0 / 60.0;

/// Maximum timestep accumulation (prevents spiral of death)
pub const MAX_TIMESTEP_ACCUMULATION: f32 = 0.25;

/// Physics body identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(pub usize);

/// Physics body representing a rigid body in the simulation
#[derive(Debug, Clone)]
pub struct PhysicsBody {
    /// Unique identifier
    pub id: BodyId,

    /// Position in world space
    pub position: Vec3,

    /// Linear velocity (m/s)
    pub velocity: Vec3,

    /// Linear acceleration (m/s²)
    pub acceleration: Vec3,

    /// Orientation (quaternion)
    pub orientation: Quat,

    /// Angular velocity (rad/s)
    pub angular_velocity: Vec3,

    /// Angular acceleration (rad/s²)
    pub angular_acceleration: Vec3,

    /// Mass (kg)
    pub mass: f32,

    /// Inverse mass (1/mass, 0 for static bodies)
    pub inv_mass: f32,

    /// Moment of inertia (simplified as scalar for 2D-ish simulation)
    pub moment_of_inertia: f32,

    /// Inverse moment of inertia
    pub inv_moment_of_inertia: f32,

    /// Accumulated forces to apply this frame
    pub force_accumulator: Vec3,

    /// Accumulated torques to apply this frame
    pub torque_accumulator: Vec3,

    /// Damping factor for linear velocity (0.0-1.0)
    pub linear_damping: f32,

    /// Damping factor for angular velocity (0.0-1.0)
    pub angular_damping: f32,

    /// Is this body static (immovable)?
    pub is_static: bool,

    /// Is this body affected by gravity?
    pub use_gravity: bool,
}

impl PhysicsBody {
    /// Create a new dynamic physics body
    pub fn new(id: BodyId, position: Vec3, mass: f32) -> Self {
        let inv_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        let moment_of_inertia = mass * 2.0; // Simplified for now
        let inv_moment = if moment_of_inertia > 0.0 {
            1.0 / moment_of_inertia
        } else {
            0.0
        };

        Self {
            id,
            position,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            orientation: Quat::IDENTITY,
            angular_velocity: Vec3::ZERO,
            angular_acceleration: Vec3::ZERO,
            mass,
            inv_mass,
            moment_of_inertia,
            inv_moment_of_inertia: inv_moment,
            force_accumulator: Vec3::ZERO,
            torque_accumulator: Vec3::ZERO,
            linear_damping: 0.99,
            angular_damping: 0.95,
            is_static: false,
            use_gravity: true,
        }
    }

    /// Create a static physics body (infinite mass)
    pub fn new_static(id: BodyId, position: Vec3) -> Self {
        Self {
            id,
            position,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            orientation: Quat::IDENTITY,
            angular_velocity: Vec3::ZERO,
            angular_acceleration: Vec3::ZERO,
            mass: 0.0,
            inv_mass: 0.0,
            moment_of_inertia: 0.0,
            inv_moment_of_inertia: 0.0,
            force_accumulator: Vec3::ZERO,
            torque_accumulator: Vec3::ZERO,
            linear_damping: 1.0,
            angular_damping: 1.0,
            is_static: true,
            use_gravity: false,
        }
    }

    /// Add a force to be applied this frame
    pub fn add_force(&mut self, force: Vec3) {
        if !self.is_static {
            self.force_accumulator += force;
        }
    }

    /// Add a force at a world position (generates torque)
    pub fn add_force_at_position(&mut self, force: Vec3, position: Vec3) {
        if self.is_static {
            return;
        }

        self.force_accumulator += force;

        // Calculate torque: r × F
        let r = position - self.position;
        let torque = r.cross(force);
        self.torque_accumulator += torque;
    }

    /// Add torque to be applied this frame
    pub fn add_torque(&mut self, torque: Vec3) {
        if !self.is_static {
            self.torque_accumulator += torque;
        }
    }

    /// Clear force and torque accumulators (call after integration)
    pub fn clear_accumulators(&mut self) {
        self.force_accumulator = Vec3::ZERO;
        self.torque_accumulator = Vec3::ZERO;
    }

    /// Set velocity directly
    pub fn set_velocity(&mut self, velocity: Vec3) {
        if !self.is_static {
            self.velocity = velocity;
        }
    }

    /// Set angular velocity directly
    pub fn set_angular_velocity(&mut self, angular_velocity: Vec3) {
        if !self.is_static {
            self.angular_velocity = angular_velocity;
        }
    }
}

/// Physics world containing all bodies and simulation state
#[derive(Debug)]
pub struct PhysicsWorld {
    /// All physics bodies
    bodies: Vec<PhysicsBody>,

    /// Gravity vector (m/s²)
    pub gravity: Vec3,

    /// Timestep accumulator for fixed timestep
    accumulator: f32,

    /// Current simulation time
    pub time: f32,

    /// Number of integration steps performed
    pub step_count: u64,
}

impl PhysicsWorld {
    /// Create a new physics world
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Vec3::new(0.0, -9.81, 0.0), // Earth gravity
            accumulator: 0.0,
            time: 0.0,
            step_count: 0,
        }
    }

    /// Create a new physics world with custom gravity
    pub fn new_with_gravity(gravity: Vec3) -> Self {
        Self {
            bodies: Vec::new(),
            gravity,
            accumulator: 0.0,
            time: 0.0,
            step_count: 0,
        }
    }

    /// Add a body to the world
    pub fn add_body(&mut self, body: PhysicsBody) -> BodyId {
        let id = body.id;
        self.bodies.push(body);
        id
    }

    /// Get a body by ID
    pub fn get_body(&self, id: BodyId) -> Option<&PhysicsBody> {
        self.bodies.iter().find(|b| b.id == id)
    }

    /// Get a mutable body by ID
    pub fn get_body_mut(&mut self, id: BodyId) -> Option<&mut PhysicsBody> {
        self.bodies.iter_mut().find(|b| b.id == id)
    }

    /// Get all bodies
    pub fn bodies(&self) -> &[PhysicsBody] {
        &self.bodies
    }

    /// Get all bodies mutably
    pub fn bodies_mut(&mut self) -> &mut [PhysicsBody] {
        &mut self.bodies
    }

    /// Remove a body from the world
    pub fn remove_body(&mut self, id: BodyId) -> Option<PhysicsBody> {
        if let Some(index) = self.bodies.iter().position(|b| b.id == id) {
            Some(self.bodies.remove(index))
        } else {
            None
        }
    }

    /// Clear all bodies
    pub fn clear(&mut self) {
        self.bodies.clear();
        self.accumulator = 0.0;
    }

    /// Update the physics simulation with variable timestep
    /// Uses fixed timestep internally for stability
    pub fn step(&mut self, delta_time: f32) {
        // Clamp delta_time to prevent spiral of death
        let dt = delta_time.min(MAX_TIMESTEP_ACCUMULATION);
        self.accumulator += dt;

        // Perform fixed timestep updates
        while self.accumulator >= PHYSICS_TIMESTEP {
            self.integrate(PHYSICS_TIMESTEP);
            self.accumulator -= PHYSICS_TIMESTEP;
            self.time += PHYSICS_TIMESTEP;
            self.step_count += 1;
        }
    }

    /// Perform one integration step
    fn integrate(&mut self, dt: f32) {
        // Apply gravity and calculate accelerations
        for body in &mut self.bodies {
            if body.is_static {
                continue;
            }

            // Apply gravity
            if body.use_gravity {
                body.add_force(self.gravity * body.mass);
            }

            // Calculate acceleration from forces: a = F/m
            body.acceleration = body.force_accumulator * body.inv_mass;

            // Calculate angular acceleration from torques: α = τ/I
            body.angular_acceleration = body.torque_accumulator * body.inv_moment_of_inertia;
        }

        // Integrate velocities and positions (Semi-implicit Euler)
        for body in &mut self.bodies {
            if body.is_static {
                continue;
            }

            // Update linear velocity: v = v + a*dt
            body.velocity += body.acceleration * dt;

            // Apply linear damping
            body.velocity *= body.linear_damping;

            // Update position: p = p + v*dt
            body.position += body.velocity * dt;

            // Update angular velocity: ω = ω + α*dt
            body.angular_velocity += body.angular_acceleration * dt;

            // Apply angular damping
            body.angular_velocity *= body.angular_damping;

            // Update orientation using exponential map
            let ang_vel = body.angular_velocity;
            let ang_mag = ang_vel.length();
            if ang_mag > 0.0001 {
                let delta_q = Quat::from_scaled_axis(ang_vel * dt);
                body.orientation = (delta_q * body.orientation).normalize();
            }

            // Clear force accumulators for next frame
            body.clear_accumulators();
        }
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_creation() {
        let body = PhysicsBody::new(BodyId(0), Vec3::ZERO, 100.0);
        assert_eq!(body.mass, 100.0);
        assert_eq!(body.inv_mass, 0.01);
        assert!(!body.is_static);
    }

    #[test]
    fn test_static_body() {
        let body = PhysicsBody::new_static(BodyId(0), Vec3::ZERO);
        assert_eq!(body.mass, 0.0);
        assert_eq!(body.inv_mass, 0.0);
        assert!(body.is_static);
    }

    #[test]
    fn test_add_force() {
        let mut body = PhysicsBody::new(BodyId(0), Vec3::ZERO, 100.0);
        body.add_force(Vec3::new(100.0, 0.0, 0.0));
        assert_eq!(body.force_accumulator.x, 100.0);
    }

    #[test]
    fn test_physics_world() {
        let mut world = PhysicsWorld::new();
        let body = PhysicsBody::new(BodyId(0), Vec3::ZERO, 100.0);
        world.add_body(body);
        assert_eq!(world.bodies().len(), 1);
    }

    #[test]
    fn test_gravity_integration() {
        let mut world = PhysicsWorld::new();
        let mut body = PhysicsBody::new(BodyId(0), Vec3::new(0.0, 10.0, 0.0), 100.0);
        body.use_gravity = true;
        world.add_body(body);

        // Step simulation
        world.step(1.0 / 60.0);

        // Check that body has fallen
        let body = world.get_body(BodyId(0)).unwrap();
        assert!(body.velocity.y < 0.0); // Should have downward velocity
    }

    #[test]
    fn test_clear_accumulators() {
        let mut body = PhysicsBody::new(BodyId(0), Vec3::ZERO, 100.0);
        body.add_force(Vec3::new(100.0, 0.0, 0.0));
        body.clear_accumulators();
        assert_eq!(body.force_accumulator, Vec3::ZERO);
    }

    #[test]
    fn orientation_integrates_with_angular_velocity() {
        let mut body = PhysicsBody::new(BodyId(1), Vec3::ZERO, 100.0);
        body.angular_velocity = Vec3::new(0.0, 1.0, 0.0); // yaw 1 rad/s

        let mut world = PhysicsWorld::new();
        world.add_body(body);
        world.step(PHYSICS_TIMESTEP);

        let updated = world.get_body(BodyId(1)).unwrap();
        // Orientation should have changed from identity
        assert!(updated.orientation != Quat::IDENTITY);
    }
}
