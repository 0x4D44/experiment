//! Track Generator
//!
//! Generates realistic F1GP circuit layouts based on famous tracks

use f1gp_port::data::track::{Track, TrackSection, RacingLine, AIBehavior, SurfaceType};
use glam::Vec3;
use std::f32::consts::PI;

/// Generate all available tracks
pub fn get_all_tracks() -> Vec<Track> {
    vec![
        generate_test_track(),
        generate_monaco(),
        generate_spa(),
        generate_monza(),
        generate_silverstone(),
    ]
}

/// Get track by index (0-based)
pub fn get_track(index: usize) -> Option<Track> {
    get_all_tracks().get(index).cloned()
}

/// Get number of available tracks
pub fn get_track_count() -> usize {
    get_all_tracks().len()
}

/// Generate simple circular test track (original)
pub fn generate_test_track() -> Track {
    let mut sections = Vec::new();
    let radius = 500.0;
    let num_segments = 64;

    for i in 0..num_segments {
        let angle = (i as f32) * 2.0 * PI / (num_segments as f32);
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        // Add some elevation variation (sine wave)
        let elevation = (angle * 2.0).sin() * 10.0;

        // Add banking in corners
        let banking_angle = (angle * 4.0).sin().abs() * 0.3;

        sections.push(TrackSection {
            position: Vec3::new(x, elevation, z),
            width: 15.0,
            banking: banking_angle,
            elevation: 0.0,
            surface: SurfaceType::Track,
            length: (2.0 * PI * radius) / (num_segments as f32),
        });
    }

    Track {
        name: "Test Circuit".to_string(),
        length: 2.0 * PI * radius,
        object_shapes: Vec::new(),
        sections,
        racing_line: RacingLine { points: Vec::new() },
        ai_behavior: AIBehavior::default(),
        pit_lane: Vec::new(),
        cameras: Vec::new(),
        checksum: 0,
    }
}

/// Generate Monaco street circuit
/// Tight, twisty, slow corners, minimal runoff
pub fn generate_monaco() -> Track {
    let mut sections = Vec::new();
    let mut x = 0.0;
    let mut z = 0.0;
    let mut heading = 0.0_f32; // Direction in radians

    // Helper to add straight section
    let add_straight = |sections: &mut Vec<TrackSection>, x: &mut f32, z: &mut f32, heading: f32, length: f32, width: f32| {
        let dx = heading.cos() * length;
        let dz = heading.sin() * length;
        *x += dx;
        *z += dz;
        sections.push(TrackSection {
            position: Vec3::new(*x, 0.0, *z),
            width,
            banking: 0.0,
            elevation: 0.0,
            surface: SurfaceType::Track,
            length,
        });
    };

    // Helper to add corner (series of small segments)
    let add_corner = |sections: &mut Vec<TrackSection>, x: &mut f32, z: &mut f32, heading: &mut f32,
                          radius: f32, angle: f32, width: f32, segments: usize| {
        let angle_per_segment = angle / (segments as f32);
        for _ in 0..segments {
            *heading += angle_per_segment;
            let dx = heading.cos() * radius * angle_per_segment;
            let dz = heading.sin() * radius * angle_per_segment;
            *x += dx;
            *z += dz;
            sections.push(TrackSection {
                position: Vec3::new(*x, 0.0, *z),
                width,
                banking: 0.0,
                elevation: 0.0,
                surface: SurfaceType::Track,
                length: radius * angle_per_segment.abs(),
            });
        }
    };

    // Monaco layout (simplified)
    // Start/Finish straight
    add_straight(&mut sections, &mut x, &mut z, heading, 200.0, 12.0);

    // Sainte Devote (right hairpin)
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 30.0, PI * 0.6, 11.0, 8);

    // Massenet/Casino uphill
    add_straight(&mut sections, &mut x, &mut z, heading, 150.0, 10.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 40.0, -PI * 0.5, 10.0, 6);

    // Casino Square
    add_straight(&mut sections, &mut x, &mut z, heading, 100.0, 11.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 35.0, PI * 0.4, 11.0, 5);

    // Mirabeau (tight right)
    add_straight(&mut sections, &mut x, &mut z, heading, 80.0, 10.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 25.0, PI * 0.7, 9.0, 7);

    // Loews/Portier
    add_straight(&mut sections, &mut x, &mut z, heading, 120.0, 11.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 30.0, -PI * 0.5, 10.0, 6);

    // Tunnel section
    add_straight(&mut sections, &mut x, &mut z, heading, 250.0, 12.0);

    // Nouvelle Chicane
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 20.0, PI * 0.4, 11.0, 4);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 20.0, -PI * 0.4, 11.0, 4);

    // Tabac corner
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 40.0, -PI * 0.4, 11.0, 5);

    // Swimming pool complex
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 30.0, PI * 0.5, 10.0, 5);
    add_straight(&mut sections, &mut x, &mut z, heading, 60.0, 10.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 30.0, -PI * 0.5, 10.0, 5);

    // Rascasse (tight hairpin)
    add_straight(&mut sections, &mut x, &mut z, heading, 80.0, 11.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 25.0, PI * 0.8, 9.0, 8);

    // Anthony Noghes final corner
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 40.0, PI * 0.4, 11.0, 5);

    // Return to start
    add_straight(&mut sections, &mut x, &mut z, heading, 150.0, 12.0);

    let length: f32 = sections.iter().map(|s| s.length).sum();

    Track {
        name: "Monaco".to_string(),
        length,
        object_shapes: Vec::new(),
        sections,
        racing_line: RacingLine { points: Vec::new() },
        ai_behavior: AIBehavior::default(),
        pit_lane: Vec::new(),
        cameras: Vec::new(),
        checksum: 0,
    }
}

/// Generate Spa-Francorchamps
/// Fast, flowing, massive elevation changes (Eau Rouge!)
pub fn generate_spa() -> Track {
    let mut sections = Vec::new();
    let mut x = 0.0;
    let mut z = 0.0;
    let mut y = 0.0;
    let mut heading = 0.0_f32;

    // Helper to add straight with elevation
    let mut add_straight = |sections: &mut Vec<TrackSection>, x: &mut f32, z: &mut f32, y: &mut f32,
                            heading: f32, length: f32, width: f32, elevation_change: f32| {
        let dx = heading.cos() * length;
        let dz = heading.sin() * length;
        *x += dx;
        *z += dz;
        *y += elevation_change;
        sections.push(TrackSection {
            position: Vec3::new(*x, *y, *z),
            width,
            banking: 0.0,
            elevation: elevation_change,
            surface: SurfaceType::Track,
            length,
        });
    };

    // Helper for corners with elevation
    let mut add_corner = |sections: &mut Vec<TrackSection>, x: &mut f32, z: &mut f32, y: &mut f32,
                          heading: &mut f32, radius: f32, angle: f32, width: f32,
                          elevation_change: f32, segments: usize| {
        let angle_per_segment = angle / (segments as f32);
        let elevation_per_segment = elevation_change / (segments as f32);
        for _ in 0..segments {
            *heading += angle_per_segment;
            let dx = heading.cos() * radius * angle_per_segment;
            let dz = heading.sin() * radius * angle_per_segment;
            *x += dx;
            *z += dz;
            *y += elevation_per_segment;
            sections.push(TrackSection {
                position: Vec3::new(*x, *y, *z),
                width,
                banking: 0.0,
                elevation: elevation_per_segment,
                surface: SurfaceType::Track,
                length: radius * angle_per_segment.abs(),
            });
        }
    };

    // Spa layout
    // Start/Finish straight
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 300.0, 14.0, 0.0);

    // La Source hairpin
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 40.0, PI, 12.0, -5.0, 10);

    // Downhill to Eau Rouge
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 200.0, 13.0, -15.0);

    // EAU ROUGE (legendary uphill left-right-left)
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 35.0, -PI * 0.3, 12.0, 5.0, 4);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 40.0, PI * 0.4, 12.0, 20.0, 5);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 35.0, -PI * 0.2, 12.0, 10.0, 4);

    // Kemmel straight (uphill continues)
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 400.0, 14.0, 15.0);

    // Les Combes (chicane)
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 50.0, PI * 0.5, 12.0, 0.0, 6);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 45.0, -PI * 0.4, 12.0, -5.0, 5);

    // Malmedy curve
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 150.0, 13.0, -5.0);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 60.0, -PI * 0.5, 13.0, -8.0, 6);

    // Rivage
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 120.0, 12.0, -10.0);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 45.0, PI * 0.6, 11.0, -5.0, 6);

    // Pouhon (long left)
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 70.0, -PI * 0.7, 13.0, 0.0, 8);

    // Campus/Stavelot
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 200.0, 13.0, 5.0);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 50.0, PI * 0.5, 12.0, 0.0, 6);

    // Blanchimont (fast left kink)
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 150.0, 14.0, 0.0);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 100.0, -PI * 0.3, 13.0, 0.0, 4);

    // Bus Stop chicane
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 100.0, 13.0, 0.0);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 25.0, PI * 0.4, 11.0, 0.0, 4);
    add_corner(&mut sections, &mut x, &mut z, &mut y, &mut heading, 25.0, -PI * 0.4, 11.0, 0.0, 4);

    // Back to start/finish
    add_straight(&mut sections, &mut x, &mut z, &mut y, heading, 250.0, 14.0, 0.0);

    let length: f32 = sections.iter().map(|s| s.length).sum();

    Track {
        name: "Spa-Francorchamps".to_string(),
        length,
        object_shapes: Vec::new(),
        sections,
        racing_line: RacingLine { points: Vec::new() },
        ai_behavior: AIBehavior::default(),
        pit_lane: Vec::new(),
        cameras: Vec::new(),
        checksum: 0,
    }
}

/// Generate Monza
/// Temple of speed - long straights, chicanes
pub fn generate_monza() -> Track {
    let mut sections = Vec::new();
    let mut x = 0.0;
    let mut z = 0.0;
    let mut heading = 0.0_f32;

    let add_straight = |sections: &mut Vec<TrackSection>, x: &mut f32, z: &mut f32, heading: f32, length: f32, width: f32| {
        let dx = heading.cos() * length;
        let dz = heading.sin() * length;
        *x += dx;
        *z += dz;
        sections.push(TrackSection {
            position: Vec3::new(*x, 0.0, *z),
            width,
            banking: 0.0,
            elevation: 0.0,
            surface: SurfaceType::Track,
            length,
        });
    };

    let add_corner = |sections: &mut Vec<TrackSection>, x: &mut f32, z: &mut f32, heading: &mut f32,
                          radius: f32, angle: f32, width: f32, segments: usize| {
        let angle_per_segment = angle / (segments as f32);
        for _ in 0..segments {
            *heading += angle_per_segment;
            let dx = heading.cos() * radius * angle_per_segment;
            let dz = heading.sin() * radius * angle_per_segment;
            *x += dx;
            *z += dz;
            sections.push(TrackSection {
                position: Vec3::new(*x, 0.0, *z),
                width,
                banking: 0.0,
                elevation: 0.0,
                surface: SurfaceType::Track,
                length: radius * angle_per_segment.abs(),
            });
        }
    };

    // Monza layout - SPEED!
    // Main straight (incredibly long)
    add_straight(&mut sections, &mut x, &mut z, heading, 600.0, 15.0);

    // Variante del Rettifilo (first chicane)
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 30.0, PI * 0.4, 12.0, 4);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 30.0, -PI * 0.4, 12.0, 4);

    // Second straight
    add_straight(&mut sections, &mut x, &mut z, heading, 350.0, 14.0);

    // Curva Grande (big fast right)
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 100.0, PI * 0.5, 14.0, 6);

    // Variante della Roggia (second chicane)
    add_straight(&mut sections, &mut x, &mut z, heading, 200.0, 13.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 25.0, -PI * 0.4, 11.0, 4);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 25.0, PI * 0.4, 11.0, 4);

    // Lesmo 1 & 2 (medium-speed rights)
    add_straight(&mut sections, &mut x, &mut z, heading, 150.0, 13.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 55.0, PI * 0.6, 12.0, 6);
    add_straight(&mut sections, &mut x, &mut z, heading, 100.0, 13.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 50.0, PI * 0.5, 12.0, 6);

    // Serraglio straight
    add_straight(&mut sections, &mut x, &mut z, heading, 250.0, 14.0);

    // Variante Ascari (fast chicane complex)
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 40.0, -PI * 0.4, 12.0, 5);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 40.0, PI * 0.3, 12.0, 4);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 35.0, -PI * 0.3, 12.0, 4);

    // Final straight section
    add_straight(&mut sections, &mut x, &mut z, heading, 180.0, 13.0);

    // Parabolica (long fast right)
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 80.0, PI, 13.0, 10);

    // Back to start/finish
    add_straight(&mut sections, &mut x, &mut z, heading, 200.0, 15.0);

    let length: f32 = sections.iter().map(|s| s.length).sum();

    Track {
        name: "Monza".to_string(),
        length,
        object_shapes: Vec::new(),
        sections,
        racing_line: RacingLine { points: Vec::new() },
        ai_behavior: AIBehavior::default(),
        pit_lane: Vec::new(),
        cameras: Vec::new(),
        checksum: 0,
    }
}

/// Generate Silverstone
/// Fast, flowing British circuit
pub fn generate_silverstone() -> Track {
    let mut sections = Vec::new();
    let mut x = 0.0;
    let mut z = 0.0;
    let mut heading = 0.0_f32;

    let add_straight = |sections: &mut Vec<TrackSection>, x: &mut f32, z: &mut f32, heading: f32, length: f32, width: f32| {
        let dx = heading.cos() * length;
        let dz = heading.sin() * length;
        *x += dx;
        *z += dz;
        sections.push(TrackSection {
            position: Vec3::new(*x, 0.0, *z),
            width,
            banking: 0.0,
            elevation: 0.0,
            surface: SurfaceType::Track,
            length,
        });
    };

    let add_corner = |sections: &mut Vec<TrackSection>, x: &mut f32, z: &mut f32, heading: &mut f32,
                          radius: f32, angle: f32, width: f32, segments: usize| {
        let angle_per_segment = angle / (segments as f32);
        for _ in 0..segments {
            *heading += angle_per_segment;
            let dx = heading.cos() * radius * angle_per_segment;
            let dz = heading.sin() * radius * angle_per_segment;
            *x += dx;
            *z += dz;
            sections.push(TrackSection {
                position: Vec3::new(*x, 0.0, *z),
                width,
                banking: 0.0,
                elevation: 0.0,
                surface: SurfaceType::Track,
                length: radius * angle_per_segment.abs(),
            });
        }
    };

    // Silverstone layout
    // Start/Finish straight
    add_straight(&mut sections, &mut x, &mut z, heading, 350.0, 14.0);

    // Abbey (fast right)
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 70.0, PI * 0.4, 13.0, 5);

    // Farm Curve (fast left)
    add_straight(&mut sections, &mut x, &mut z, heading, 100.0, 13.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 80.0, -PI * 0.5, 13.0, 6);

    // Village complex (slow)
    add_straight(&mut sections, &mut x, &mut z, heading, 120.0, 12.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 35.0, PI * 0.6, 11.0, 6);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 40.0, -PI * 0.4, 11.0, 5);

    // The Loop (Wellington Straight + Brooklands)
    add_straight(&mut sections, &mut x, &mut z, heading, 300.0, 13.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 60.0, PI * 0.7, 12.0, 7);

    // Luffield
    add_straight(&mut sections, &mut x, &mut z, heading, 100.0, 12.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 45.0, PI * 0.6, 12.0, 6);

    // Woodcote/Copse straight section
    add_straight(&mut sections, &mut x, &mut z, heading, 250.0, 13.0);

    // Copse (fast right)
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 90.0, PI * 0.5, 13.0, 6);

    // Maggots-Becketts complex (fast flowing)
    add_straight(&mut sections, &mut x, &mut z, heading, 100.0, 13.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 70.0, -PI * 0.4, 13.0, 5);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 75.0, PI * 0.5, 13.0, 6);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 65.0, -PI * 0.3, 13.0, 4);

    // Hangar Straight
    add_straight(&mut sections, &mut x, &mut z, heading, 400.0, 14.0);

    // Stowe (medium-speed right)
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 55.0, PI * 0.5, 12.0, 6);

    // Vale/Club chicane
    add_straight(&mut sections, &mut x, &mut z, heading, 150.0, 13.0);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 40.0, -PI * 0.4, 12.0, 5);
    add_corner(&mut sections, &mut x, &mut z, &mut heading, 40.0, PI * 0.5, 12.0, 6);

    // Back to start/finish
    add_straight(&mut sections, &mut x, &mut z, heading, 200.0, 14.0);

    let length: f32 = sections.iter().map(|s| s.length).sum();

    Track {
        name: "Silverstone".to_string(),
        length,
        object_shapes: Vec::new(),
        sections,
        racing_line: RacingLine { points: Vec::new() },
        ai_behavior: AIBehavior::default(),
        pit_lane: Vec::new(),
        cameras: Vec::new(),
        checksum: 0,
    }
}
