use platformer_rust::physics::*;

#[test]
fn test_aabb_creation() {
    let bbox = AABB::new(10.0, 20.0, 30.0, 40.0);
    assert_eq!(bbox.x, 10.0);
    assert_eq!(bbox.y, 20.0);
    assert_eq!(bbox.width, 30.0);
    assert_eq!(bbox.height, 40.0);
}

#[test]
fn test_aabb_intersection_overlapping() {
    let box1 = AABB::new(0.0, 0.0, 20.0, 20.0);
    let box2 = AABB::new(10.0, 10.0, 20.0, 20.0);

    assert!(box1.intersects(&box2));
    assert!(box2.intersects(&box1));
}

#[test]
fn test_aabb_intersection_non_overlapping() {
    let box1 = AABB::new(0.0, 0.0, 10.0, 10.0);
    let box2 = AABB::new(20.0, 20.0, 10.0, 10.0);

    assert!(!box1.intersects(&box2));
    assert!(!box2.intersects(&box1));
}

#[test]
fn test_aabb_intersection_touching() {
    let box1 = AABB::new(0.0, 0.0, 10.0, 10.0);
    let box2 = AABB::new(10.0, 0.0, 10.0, 10.0);

    // Touching edges should not be considered intersecting
    assert!(!box1.intersects(&box2));
}

#[test]
fn test_aabb_contains_point() {
    let bbox = AABB::new(10.0, 10.0, 20.0, 20.0);

    assert!(bbox.contains_point(15.0, 15.0)); // Inside
    assert!(bbox.contains_point(10.0, 10.0)); // Top-left corner
    assert!(bbox.contains_point(30.0, 30.0)); // Bottom-right corner
    assert!(bbox.contains_point(20.0, 20.0)); // Middle
    assert!(!bbox.contains_point(5.0, 5.0));  // Outside
    assert!(!bbox.contains_point(35.0, 35.0)); // Outside
}

#[test]
fn test_aabb_center() {
    let bbox = AABB::new(10.0, 20.0, 30.0, 40.0);
    let center = bbox.center();

    assert_eq!(center.x, 25.0);
    assert_eq!(center.y, 40.0);
}

#[test]
fn test_aabb_overlap_x() {
    let box1 = AABB::new(0.0, 0.0, 20.0, 20.0);
    let box2 = AABB::new(10.0, 0.0, 20.0, 20.0);

    assert_eq!(box1.overlap_x(&box2), 10.0);
    assert_eq!(box2.overlap_x(&box1), 10.0);
}

#[test]
fn test_aabb_overlap_y() {
    let box1 = AABB::new(0.0, 0.0, 20.0, 20.0);
    let box2 = AABB::new(0.0, 10.0, 20.0, 20.0);

    assert_eq!(box1.overlap_y(&box2), 10.0);
    assert_eq!(box2.overlap_y(&box1), 10.0);
}

#[test]
fn test_aabb_no_overlap() {
    let box1 = AABB::new(0.0, 0.0, 10.0, 10.0);
    let box2 = AABB::new(20.0, 20.0, 10.0, 10.0);

    assert_eq!(box1.overlap_x(&box2), 0.0);
    assert_eq!(box1.overlap_y(&box2), 0.0);
}

#[test]
fn test_physics_body_creation() {
    let body = PhysicsBody::new(10.0, 20.0, 30.0, 40.0);

    assert_eq!(body.position.x, 10.0);
    assert_eq!(body.position.y, 20.0);
    assert_eq!(body.size.x, 30.0);
    assert_eq!(body.size.y, 40.0);
    assert_eq!(body.velocity.x, 0.0);
    assert_eq!(body.velocity.y, 0.0);
    assert!(!body.on_ground);
}

#[test]
fn test_physics_body_gravity() {
    let mut body = PhysicsBody::new(0.0, 0.0, 10.0, 10.0);

    body.apply_gravity(0.016); // ~60 FPS frame

    assert!(body.velocity.y > 0.0); // Falling down
    assert_eq!(body.velocity.x, 0.0); // No horizontal velocity
}

#[test]
fn test_physics_body_gravity_capped() {
    let mut body = PhysicsBody::new(0.0, 0.0, 10.0, 10.0);

    // Apply gravity for many frames
    for _ in 0..1000 {
        body.apply_gravity(0.016);
    }

    // Velocity should be capped at MAX_FALL_SPEED
    assert!(body.velocity.y <= MAX_FALL_SPEED);
}

#[test]
fn test_physics_body_update_position() {
    let mut body = PhysicsBody::new(0.0, 0.0, 10.0, 10.0);
    body.velocity.x = 100.0;
    body.velocity.y = 50.0;

    body.update_position(1.0); // 1 second

    assert_eq!(body.position.x, 100.0);
    assert_eq!(body.position.y, 50.0);
}

#[test]
fn test_collision_resolution_horizontal() {
    let moving = AABB::new(15.0, 10.0, 10.0, 10.0);
    let stationary = AABB::new(20.0, 10.0, 10.0, 10.0);

    let result = resolve_collision(&moving, &stationary);

    assert!(result.collided);
    assert_eq!(result.normal.x, -1.0); // Collision from left
    assert_eq!(result.normal.y, 0.0);
    assert_eq!(result.penetration, 5.0);
}

#[test]
fn test_collision_resolution_vertical() {
    let moving = AABB::new(10.0, 15.0, 10.0, 10.0);
    let stationary = AABB::new(10.0, 20.0, 10.0, 10.0);

    let result = resolve_collision(&moving, &stationary);

    assert!(result.collided);
    assert_eq!(result.normal.x, 0.0);
    assert_eq!(result.normal.y, -1.0); // Collision from top
    assert_eq!(result.penetration, 5.0);
}

#[test]
fn test_collision_resolution_no_collision() {
    let moving = AABB::new(0.0, 0.0, 10.0, 10.0);
    let stationary = AABB::new(20.0, 20.0, 10.0, 10.0);

    let result = resolve_collision(&moving, &stationary);

    assert!(!result.collided);
}

#[test]
fn test_collision_from_different_sides() {
    let stationary = AABB::new(50.0, 50.0, 20.0, 20.0);

    // From left
    let from_left = AABB::new(45.0, 55.0, 10.0, 10.0);
    let result = resolve_collision(&from_left, &stationary);
    assert!(result.collided);
    assert_eq!(result.normal.x, -1.0);

    // From right
    let from_right = AABB::new(65.0, 55.0, 10.0, 10.0);
    let result = resolve_collision(&from_right, &stationary);
    assert!(result.collided);
    assert_eq!(result.normal.x, 1.0);

    // From top
    let from_top = AABB::new(55.0, 45.0, 10.0, 10.0);
    let result = resolve_collision(&from_top, &stationary);
    assert!(result.collided);
    assert_eq!(result.normal.y, -1.0);

    // From bottom
    let from_bottom = AABB::new(55.0, 65.0, 10.0, 10.0);
    let result = resolve_collision(&from_bottom, &stationary);
    assert!(result.collided);
    assert_eq!(result.normal.y, 1.0);
}

#[test]
fn test_gravity_scale() {
    let mut body1 = PhysicsBody::new(0.0, 0.0, 10.0, 10.0);
    body1.gravity_scale = 1.0;

    let mut body2 = PhysicsBody::new(0.0, 0.0, 10.0, 10.0);
    body2.gravity_scale = 0.5;

    body1.apply_gravity(0.016);
    body2.apply_gravity(0.016);

    // Body with lower gravity scale should fall slower
    assert!(body2.velocity.y < body1.velocity.y);
}

#[test]
fn test_zero_gravity() {
    let mut body = PhysicsBody::new(0.0, 0.0, 10.0, 10.0);
    body.gravity_scale = 0.0;

    body.apply_gravity(0.016);

    assert_eq!(body.velocity.y, 0.0); // No gravity
}
