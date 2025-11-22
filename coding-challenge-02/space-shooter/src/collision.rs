use macroquad::prelude::*;

/// Check collision between two rectangles
pub fn check_collision(rect1: Rect, rect2: Rect) -> bool {
    rect1.overlaps(&rect2)
}

/// Check collision between point and rectangle
pub fn point_in_rect(point: Vec2, rect: Rect) -> bool {
    point.x >= rect.x
        && point.x <= rect.x + rect.w
        && point.y >= rect.y
        && point.y <= rect.y + rect.h
}

/// Check circular collision (more accurate for round objects)
pub fn check_circular_collision(pos1: Vec2, radius1: f32, pos2: Vec2, radius2: f32) -> bool {
    let distance = pos1.distance(pos2);
    distance < (radius1 + radius2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_collision() {
        let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect2 = Rect::new(5.0, 5.0, 10.0, 10.0);
        assert!(check_collision(rect1, rect2));

        let rect3 = Rect::new(20.0, 20.0, 10.0, 10.0);
        assert!(!check_collision(rect1, rect3));
    }

    #[test]
    fn test_point_in_rect() {
        let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(point_in_rect(vec2(5.0, 5.0), rect));
        assert!(!point_in_rect(vec2(15.0, 15.0), rect));
    }

    #[test]
    fn test_circular_collision() {
        assert!(check_circular_collision(vec2(0.0, 0.0), 5.0, vec2(8.0, 0.0), 5.0));
        assert!(!check_circular_collision(vec2(0.0, 0.0), 5.0, vec2(20.0, 0.0), 5.0));
    }
}
