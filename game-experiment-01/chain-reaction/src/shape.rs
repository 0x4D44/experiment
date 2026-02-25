use crate::vec2::Vec2;

/// Axis-Aligned Bounding Box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        AABB { min, max }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half_size = size * 0.5;
        AABB {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    pub fn expand(&self, amount: f64) -> AABB {
        AABB {
            min: self.min - Vec2::new(amount, amount),
            max: self.max + Vec2::new(amount, amount),
        }
    }

    pub fn merge(&self, other: &AABB) -> AABB {
        AABB {
            min: Vec2::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: Vec2::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        }
    }
}

/// Circle shape
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Vec2, radius: f64) -> Self {
        Circle { center, radius }
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        self.center.distance_squared(&point) <= self.radius * self.radius
    }

    pub fn to_aabb(&self) -> AABB {
        let r = Vec2::new(self.radius, self.radius);
        AABB {
            min: self.center - r,
            max: self.center + r,
        }
    }
}

/// Oriented Bounding Box (rotated rectangle)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OBB {
    pub center: Vec2,
    pub half_extents: Vec2,
    pub rotation: f64,
}

impl OBB {
    pub fn new(center: Vec2, half_extents: Vec2, rotation: f64) -> Self {
        OBB {
            center,
            half_extents,
            rotation,
        }
    }

    pub fn to_aabb(&self) -> AABB {
        let cos = self.rotation.cos().abs();
        let sin = self.rotation.sin().abs();
        let half_width = self.half_extents.x * cos + self.half_extents.y * sin;
        let half_height = self.half_extents.x * sin + self.half_extents.y * cos;

        AABB {
            min: self.center - Vec2::new(half_width, half_height),
            max: self.center + Vec2::new(half_width, half_height),
        }
    }

    pub fn get_vertices(&self) -> [Vec2; 4] {
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        let x_axis = Vec2::new(cos, sin);
        let y_axis = Vec2::new(-sin, cos);

        [
            self.center + x_axis * self.half_extents.x + y_axis * self.half_extents.y,
            self.center - x_axis * self.half_extents.x + y_axis * self.half_extents.y,
            self.center - x_axis * self.half_extents.x - y_axis * self.half_extents.y,
            self.center + x_axis * self.half_extents.x - y_axis * self.half_extents.y,
        ]
    }
}

/// Shape types for collision detection
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle(Circle),
    Box(OBB),
}

impl Shape {
    pub fn to_aabb(&self) -> AABB {
        match self {
            Shape::Circle(c) => c.to_aabb(),
            Shape::Box(obb) => obb.to_aabb(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_creation() {
        let aabb = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert_eq!(aabb.center(), Vec2::new(5.0, 5.0));
        assert_eq!(aabb.size(), Vec2::new(10.0, 10.0));
    }

    #[test]
    fn test_aabb_from_center_size() {
        let aabb = AABB::from_center_size(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        assert_eq!(aabb.min, Vec2::new(0.0, 0.0));
        assert_eq!(aabb.max, Vec2::new(10.0, 10.0));
    }

    #[test]
    fn test_aabb_contains_point() {
        let aabb = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(aabb.contains_point(Vec2::new(5.0, 5.0)));
        assert!(aabb.contains_point(Vec2::new(0.0, 0.0)));
        assert!(aabb.contains_point(Vec2::new(10.0, 10.0)));
        assert!(!aabb.contains_point(Vec2::new(-1.0, 5.0)));
        assert!(!aabb.contains_point(Vec2::new(11.0, 5.0)));
    }

    #[test]
    fn test_aabb_intersection() {
        let aabb1 = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let aabb2 = AABB::new(Vec2::new(5.0, 5.0), Vec2::new(15.0, 15.0));
        let aabb3 = AABB::new(Vec2::new(20.0, 20.0), Vec2::new(30.0, 30.0));

        assert!(aabb1.intersects(&aabb2));
        assert!(aabb2.intersects(&aabb1));
        assert!(!aabb1.intersects(&aabb3));
        assert!(!aabb3.intersects(&aabb1));
    }

    #[test]
    fn test_circle_creation() {
        let circle = Circle::new(Vec2::new(5.0, 5.0), 3.0);
        assert_eq!(circle.center, Vec2::new(5.0, 5.0));
        assert_eq!(circle.radius, 3.0);
    }

    #[test]
    fn test_circle_contains_point() {
        let circle = Circle::new(Vec2::new(0.0, 0.0), 5.0);
        assert!(circle.contains_point(Vec2::new(0.0, 0.0)));
        assert!(circle.contains_point(Vec2::new(3.0, 4.0)));
        assert!(circle.contains_point(Vec2::new(5.0, 0.0)));
        assert!(!circle.contains_point(Vec2::new(4.0, 4.0)));
    }

    #[test]
    fn test_circle_to_aabb() {
        let circle = Circle::new(Vec2::new(5.0, 5.0), 3.0);
        let aabb = circle.to_aabb();
        assert_eq!(aabb.min, Vec2::new(2.0, 2.0));
        assert_eq!(aabb.max, Vec2::new(8.0, 8.0));
    }

    #[test]
    fn test_obb_vertices() {
        let obb = OBB::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 1.0), 0.0);
        let verts = obb.get_vertices();

        assert!((verts[0].x - 2.0).abs() < 1e-10);
        assert!((verts[0].y - 1.0).abs() < 1e-10);
        assert!((verts[1].x - (-2.0)).abs() < 1e-10);
        assert!((verts[1].y - 1.0).abs() < 1e-10);
        assert!((verts[2].x - (-2.0)).abs() < 1e-10);
        assert!((verts[2].y - (-1.0)).abs() < 1e-10);
        assert!((verts[3].x - 2.0).abs() < 1e-10);
        assert!((verts[3].y - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_obb_to_aabb() {
        let obb = OBB::new(Vec2::new(5.0, 5.0), Vec2::new(2.0, 1.0), 0.0);
        let aabb = obb.to_aabb();

        assert!((aabb.min.x - 3.0).abs() < 1e-10);
        assert!((aabb.min.y - 4.0).abs() < 1e-10);
        assert!((aabb.max.x - 7.0).abs() < 1e-10);
        assert!((aabb.max.y - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_aabb_merge() {
        let aabb1 = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let aabb2 = AABB::new(Vec2::new(5.0, 5.0), Vec2::new(15.0, 15.0));
        let merged = aabb1.merge(&aabb2);

        assert_eq!(merged.min, Vec2::new(0.0, 0.0));
        assert_eq!(merged.max, Vec2::new(15.0, 15.0));
    }
}
