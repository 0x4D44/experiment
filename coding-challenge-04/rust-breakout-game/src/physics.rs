/// 2D Vector for positions and velocities
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

/// Rectangle for collision detection
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    #[allow(dead_code)]
    pub fn contains_point(&self, point: &Vec2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

/// Circle for ball collision
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Check collision with rectangle and return collision normal if colliding
    pub fn collides_rect(&self, rect: &Rect) -> Option<Vec2> {
        // Find closest point on rectangle to circle center
        let closest_x = self.center.x.clamp(rect.x, rect.x + rect.width);
        let closest_y = self.center.y.clamp(rect.y, rect.y + rect.height);
        let closest = Vec2::new(closest_x, closest_y);

        // Calculate distance
        let distance = Vec2::new(
            self.center.x - closest.x,
            self.center.y - closest.y,
        );
        let dist_squared = distance.x * distance.x + distance.y * distance.y;

        if dist_squared < self.radius * self.radius {
            // Calculate normal
            if dist_squared > 0.0 {
                let dist = dist_squared.sqrt();
                Some(Vec2::new(distance.x / dist, distance.y / dist))
            } else {
                // Ball center is inside rectangle
                // Determine which edge is closest
                let left_dist = (self.center.x - rect.x).abs();
                let right_dist = (rect.x + rect.width - self.center.x).abs();
                let top_dist = (self.center.y - rect.y).abs();
                let bottom_dist = (rect.y + rect.height - self.center.y).abs();

                let min_dist = left_dist.min(right_dist).min(top_dist).min(bottom_dist);

                if min_dist == left_dist {
                    Some(Vec2::new(-1.0, 0.0))
                } else if min_dist == right_dist {
                    Some(Vec2::new(1.0, 0.0))
                } else if min_dist == top_dist {
                    Some(Vec2::new(0.0, -1.0))
                } else {
                    Some(Vec2::new(0.0, 1.0))
                }
            }
        } else {
            None
        }
    }
}

/// Reflect a velocity vector off a surface with given normal
pub fn reflect_velocity(velocity: Vec2, normal: Vec2) -> Vec2 {
    let dot = velocity.dot(&normal);
    Vec2::new(
        velocity.x - 2.0 * dot * normal.x,
        velocity.y - 2.0 * dot * normal.y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalize();
        assert!((n.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let r2 = Rect::new(5.0, 5.0, 10.0, 10.0);
        let r3 = Rect::new(20.0, 20.0, 10.0, 10.0);

        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_circle_rect_collision() {
        let circle = Circle::new(Vec2::new(5.0, 5.0), 1.0);
        let rect = Rect::new(0.0, 0.0, 10.0, 10.0);

        assert!(circle.collides_rect(&rect).is_some());

        let circle2 = Circle::new(Vec2::new(20.0, 20.0), 1.0);
        assert!(circle2.collides_rect(&rect).is_none());
    }

    #[test]
    fn test_reflect_velocity() {
        let velocity = Vec2::new(1.0, -1.0);
        let normal = Vec2::new(0.0, 1.0);
        let reflected = reflect_velocity(velocity, normal);

        assert!((reflected.x - 1.0).abs() < 0.001);
        assert!((reflected.y - 1.0).abs() < 0.001);
    }
}
