//! Field of View calculation using shadowcasting algorithm

use crate::entity::Position;
use crate::map::Map;
use std::collections::HashSet;

const FOV_RADIUS: i32 = 10;

/// Calculate visible tiles from a given position using shadowcasting
pub fn calculate_fov(map: &Map, origin: &Position, radius: i32) -> HashSet<Position> {
    let mut visible = HashSet::new();

    // Origin is always visible
    visible.insert(*origin);

    // Cast shadows in 8 octants
    for octant in 0..8 {
        cast_light(map, origin, radius, 1, 1.0, 0.0, octant, &mut visible);
    }

    visible
}

/// Recursive shadowcasting for one octant
#[allow(clippy::too_many_arguments)]
fn cast_light(
    map: &Map,
    origin: &Position,
    radius: i32,
    row: i32,
    start_slope: f32,
    end_slope: f32,
    octant: i32,
    visible: &mut HashSet<Position>,
) {
    if start_slope < end_slope {
        return;
    }

    let radius_squared = radius * radius;
    let mut next_start_slope = start_slope;

    for i in row..=radius {
        let mut blocked = false;
        let dy = -i;

        let min_col = ((start_slope * dy as f32) as i32).max(0);
        let max_col = ((end_slope * dy as f32) as i32).min(0);

        for dx in min_col..=max_col {
            let current_pos = transform_octant(origin, dx, dy, octant);

            // Check if within map bounds
            if current_pos.x < 0
                || current_pos.x >= map.width
                || current_pos.y < 0
                || current_pos.y >= map.height
            {
                continue;
            }

            let distance = dx * dx + dy * dy;
            if distance > radius_squared {
                continue;
            }

            visible.insert(current_pos);

            if blocked {
                if map.blocks_sight(current_pos.x, current_pos.y) {
                    next_start_slope = get_slope(dx, dy, false);
                } else {
                    blocked = false;
                }
            } else if map.blocks_sight(current_pos.x, current_pos.y) {
                blocked = true;
                cast_light(
                    map,
                    origin,
                    radius,
                    i + 1,
                    next_start_slope,
                    get_slope(dx, dy, true),
                    octant,
                    visible,
                );
                next_start_slope = get_slope(dx, dy, false);
            }
        }

        if blocked {
            break;
        }
    }
}

fn transform_octant(origin: &Position, col: i32, row: i32, octant: i32) -> Position {
    match octant {
        0 => Position::new(origin.x + col, origin.y + row),
        1 => Position::new(origin.x + row, origin.y + col),
        2 => Position::new(origin.x + row, origin.y - col),
        3 => Position::new(origin.x + col, origin.y - row),
        4 => Position::new(origin.x - col, origin.y - row),
        5 => Position::new(origin.x - row, origin.y - col),
        6 => Position::new(origin.x - row, origin.y + col),
        7 => Position::new(origin.x - col, origin.y + row),
        _ => *origin,
    }
}

fn get_slope(col: i32, row: i32, start: bool) -> f32 {
    if start {
        (col as f32 - 0.5) / (row as f32 + 0.5)
    } else {
        (col as f32 + 0.5) / (row as f32 - 0.5)
    }
}

pub fn default_fov_radius() -> i32 {
    FOV_RADIUS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Map;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_fov_origin_visible() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);
        let origin = map.first_room_center().unwrap();

        let visible = calculate_fov(&map, &origin, 5);
        assert!(visible.contains(&origin));
    }

    #[test]
    fn test_fov_radius() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = Map::generate(1, &mut rng);
        let origin = map.first_room_center().unwrap();

        let visible = calculate_fov(&map, &origin, 3);

        // All visible tiles should be within radius
        for pos in visible.iter() {
            let distance = origin.distance_to(pos);
            assert!(distance <= 3.5); // Allow some tolerance for rounding
        }
    }

    #[test]
    fn test_fov_blocked_by_walls() {
        let map = Map::new(10, 10);
        let origin = Position::new(5, 5);

        // In an all-wall map, only origin should be visible
        let visible = calculate_fov(&map, &origin, 5);

        // Origin is always visible
        assert!(visible.contains(&origin));

        // Most other positions should be blocked
        assert!(visible.len() < 10); // Much smaller than full radius
    }

    #[test]
    fn test_transform_octant() {
        let origin = Position::new(10, 10);

        let pos0 = transform_octant(&origin, 1, -1, 0);
        assert_eq!(pos0.x, 11);
        assert_eq!(pos0.y, 9);

        let pos4 = transform_octant(&origin, 1, -1, 4);
        assert_eq!(pos4.x, 9);
        assert_eq!(pos4.y, 11);
    }

    #[test]
    fn test_get_slope() {
        let slope1 = get_slope(1, 1, true);
        let slope2 = get_slope(1, 1, false);
        assert!(slope1 < slope2);
    }
}
