use macroquad::prelude::*;

/// Gravity constant (pixels per second squared)
pub const GRAVITY: f32 = 1200.0;

/// Maximum fall speed (pixels per second)
pub const MAX_FALL_SPEED: f32 = 600.0;

/// Player movement speed (pixels per second)
pub const PLAYER_SPEED: f32 = 200.0;

/// Jump velocity (pixels per second)
pub const JUMP_VELOCITY: f32 = -450.0;

/// Double jump velocity (slightly less powerful)
pub const DOUBLE_JUMP_VELOCITY: f32 = -400.0;

/// Represents a 2D bounding box for collision detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl AABB {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if this AABB intersects with another
    pub fn intersects(&self, other: &AABB) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Check if this AABB contains a point
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Get the center point of the AABB
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Get overlap on X axis (returns 0 if no overlap)
    pub fn overlap_x(&self, other: &AABB) -> f32 {
        let left = self.x.max(other.x);
        let right = (self.x + self.width).min(other.x + other.width);
        (right - left).max(0.0)
    }

    /// Get overlap on Y axis (returns 0 if no overlap)
    pub fn overlap_y(&self, other: &AABB) -> f32 {
        let top = self.y.max(other.y);
        let bottom = (self.y + self.height).min(other.y + other.height);
        (bottom - top).max(0.0)
    }
}

/// Physics body with position, velocity, and collision box
#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Vec2,
    pub on_ground: bool,
    pub gravity_scale: f32,
}

impl PhysicsBody {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            size: Vec2::new(width, height),
            on_ground: false,
            gravity_scale: 1.0,
        }
    }

    /// Get the AABB for this physics body
    pub fn aabb(&self) -> AABB {
        AABB::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Apply gravity to the body
    pub fn apply_gravity(&mut self, delta_time: f32) {
        self.velocity.y += GRAVITY * self.gravity_scale * delta_time;
        self.velocity.y = self.velocity.y.min(MAX_FALL_SPEED);
    }

    /// Update position based on velocity
    pub fn update_position(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }
}

/// Collision resolution information
#[derive(Debug, Clone, Copy)]
pub struct CollisionResult {
    pub collided: bool,
    pub normal: Vec2,
    pub penetration: f32,
}

impl CollisionResult {
    pub fn none() -> Self {
        Self {
            collided: false,
            normal: Vec2::ZERO,
            penetration: 0.0,
        }
    }

    pub fn new(normal: Vec2, penetration: f32) -> Self {
        Self {
            collided: true,
            normal,
            penetration,
        }
    }
}

/// Resolve collision between two AABBs
pub fn resolve_collision(moving: &AABB, stationary: &AABB) -> CollisionResult {
    if !moving.intersects(stationary) {
        return CollisionResult::none();
    }

    let overlap_x = moving.overlap_x(stationary);
    let overlap_y = moving.overlap_y(stationary);

    // Resolve on the axis with the smallest overlap
    if overlap_x < overlap_y {
        // Horizontal collision
        let normal = if moving.center().x < stationary.center().x {
            Vec2::new(-1.0, 0.0) // Collision from left
        } else {
            Vec2::new(1.0, 0.0) // Collision from right
        };
        CollisionResult::new(normal, overlap_x)
    } else {
        // Vertical collision
        let normal = if moving.center().y < stationary.center().y {
            Vec2::new(0.0, -1.0) // Collision from top
        } else {
            Vec2::new(0.0, 1.0) // Collision from bottom
        };
        CollisionResult::new(normal, overlap_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_intersection() {
        let box1 = AABB::new(0.0, 0.0, 10.0, 10.0);
        let box2 = AABB::new(5.0, 5.0, 10.0, 10.0);
        let box3 = AABB::new(20.0, 20.0, 10.0, 10.0);

        assert!(box1.intersects(&box2));
        assert!(box2.intersects(&box1));
        assert!(!box1.intersects(&box3));
    }

    #[test]
    fn test_aabb_contains_point() {
        let bbox = AABB::new(10.0, 10.0, 20.0, 20.0);

        assert!(bbox.contains_point(15.0, 15.0));
        assert!(bbox.contains_point(10.0, 10.0));
        assert!(bbox.contains_point(30.0, 30.0));
        assert!(!bbox.contains_point(5.0, 5.0));
        assert!(!bbox.contains_point(35.0, 35.0));
    }

    #[test]
    fn test_collision_resolution() {
        let moving = AABB::new(10.0, 10.0, 10.0, 10.0);
        let stationary = AABB::new(15.0, 10.0, 10.0, 10.0);

        let result = resolve_collision(&moving, &stationary);
        assert!(result.collided);
        assert_eq!(result.normal.x, -1.0);
        assert_eq!(result.normal.y, 0.0);
    }

    #[test]
    fn test_physics_body_gravity() {
        let mut body = PhysicsBody::new(0.0, 0.0, 10.0, 10.0);
        body.apply_gravity(0.016); // ~60fps

        assert!(body.velocity.y > 0.0);
    }
}
