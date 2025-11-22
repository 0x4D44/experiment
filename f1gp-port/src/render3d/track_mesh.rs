// Track Mesh Generator - Stage 6.2
// Generates 3D mesh from track data

use crate::data::{SurfaceType, Track, TrackSection};
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

/// Vertex for track rendering (with normals for lighting)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TrackVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

impl TrackVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TrackVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    /// Create vertex from Vec3 positions and normals
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2, color: [f32; 4]) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
            uv: uv.to_array(),
            color,
        }
    }
}

/// Track mesh data for rendering
pub struct TrackMesh {
    pub vertices: Vec<TrackVertex>,
    pub indices: Vec<u32>,
    pub bounds_min: Vec3,
    pub bounds_max: Vec3,
}

impl TrackMesh {
    /// Generate track mesh from track data
    pub fn from_track(track: &Track) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut bounds_min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut bounds_max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        // Generate mesh for each section
        let mut current_height = 0.0;
        for (i, section) in track.sections.iter().enumerate() {
            let next_section = track.sections.get(i + 1).or_else(|| track.sections.first());

            if let Some(next) = next_section {
                Self::generate_section_mesh(
                    section,
                    next,
                    &mut current_height,
                    &mut vertices,
                    &mut indices,
                    &mut bounds_min,
                    &mut bounds_max,
                );
            }
        }

        // Calculate normals
        Self::calculate_normals(&mut vertices, &indices);

        Self {
            vertices,
            indices,
            bounds_min,
            bounds_max,
        }
    }

    /// Generate mesh for a single track section
    fn generate_section_mesh(
        section: &TrackSection,
        next_section: &TrackSection,
        current_height: &mut f32,
        vertices: &mut Vec<TrackVertex>,
        indices: &mut Vec<u32>,
        bounds_min: &mut Vec3,
        bounds_max: &mut Vec3,
    ) {
        let base_index = vertices.len() as u32;

        // Update cumulative height
        *current_height += section.elevation;

        // Section positions
        let pos1 = section.position + Vec3::Y * *current_height;
        let pos2 = next_section.position + Vec3::Y * (*current_height + next_section.elevation);

        // Calculate perpendicular direction for track width
        let forward = (pos2 - pos1).normalize();
        let right = Vec3::new(-forward.z, 0.0, forward.x).normalize();

        // Half widths
        let half_width1 = section.width * 0.5;
        let half_width2 = next_section.width * 0.5;

        // Banking rotation (simplified - rotate around forward axis)
        let banking_offset1 = right * section.banking.tan() * half_width1;
        let banking_offset2 = right * next_section.banking.tan() * half_width2;

        // Generate vertices for road surface
        let left1 = pos1 - right * half_width1 + banking_offset1;
        let right1 = pos1 + right * half_width1 + banking_offset1;
        let left2 = pos2 - right * half_width2 + banking_offset2;
        let right2 = pos2 + right * half_width2 + banking_offset2;

        // Surface color based on type
        let color = Self::surface_color(section.surface);

        // Create vertices (quad for this section)
        let verts = [
            TrackVertex::new(left1, Vec3::Y, Vec2::new(0.0, 0.0), color),
            TrackVertex::new(right1, Vec3::Y, Vec2::new(1.0, 0.0), color),
            TrackVertex::new(left2, Vec3::Y, Vec2::new(0.0, 1.0), color),
            TrackVertex::new(right2, Vec3::Y, Vec2::new(1.0, 1.0), color),
        ];

        // Update bounds
        for v in &verts {
            let pos = Vec3::from_array(v.position);
            *bounds_min = bounds_min.min(pos);
            *bounds_max = bounds_max.max(pos);
        }

        vertices.extend_from_slice(&verts);

        // Create triangles (two per quad)
        // Triangle 1: left1, right1, left2
        indices.push(base_index);
        indices.push(base_index + 1);
        indices.push(base_index + 2);

        // Triangle 2: right1, right2, left2
        indices.push(base_index + 1);
        indices.push(base_index + 3);
        indices.push(base_index + 2);

        // Generate kerbs if this is a track section
        if section.surface == SurfaceType::Track {
            Self::generate_kerbs(
                section,
                next_section,
                pos1,
                pos2,
                forward,
                right,
                *current_height,
                vertices,
                indices,
            );
        }
    }

    /// Generate kerb geometry at track edges
    fn generate_kerbs(
        section: &TrackSection,
        next_section: &TrackSection,
        pos1: Vec3,
        pos2: Vec3,
        _forward: Vec3,
        right: Vec3,
        _height: f32,
        vertices: &mut Vec<TrackVertex>,
        indices: &mut Vec<u32>,
    ) {
        let kerb_width = 0.3; // 30cm kerbs
        let kerb_height = 0.05; // 5cm raised

        let half_width1 = section.width * 0.5;
        let half_width2 = next_section.width * 0.5;

        // Red/white kerb pattern (simplified - just red for now)
        let kerb_color = [1.0, 0.2, 0.2, 1.0]; // Red

        // Left kerb
        let left_inner1 = pos1 - right * half_width1;
        let left_outer1 = pos1 - right * (half_width1 + kerb_width);
        let left_inner2 = pos2 - right * half_width2;
        let left_outer2 = pos2 - right * (half_width2 + kerb_width);

        let base_index = vertices.len() as u32;

        vertices.extend_from_slice(&[
            TrackVertex::new(
                left_inner1 + Vec3::Y * kerb_height,
                Vec3::Y,
                Vec2::ZERO,
                kerb_color,
            ),
            TrackVertex::new(
                left_outer1 + Vec3::Y * kerb_height,
                Vec3::Y,
                Vec2::ZERO,
                kerb_color,
            ),
            TrackVertex::new(
                left_inner2 + Vec3::Y * kerb_height,
                Vec3::Y,
                Vec2::ZERO,
                kerb_color,
            ),
            TrackVertex::new(
                left_outer2 + Vec3::Y * kerb_height,
                Vec3::Y,
                Vec2::ZERO,
                kerb_color,
            ),
        ]);

        indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index + 1,
            base_index + 3,
            base_index + 2,
        ]);

        // Right kerb (similar)
        let right_inner1 = pos1 + right * half_width1;
        let right_outer1 = pos1 + right * (half_width1 + kerb_width);
        let right_inner2 = pos2 + right * half_width2;
        let right_outer2 = pos2 + right * (half_width2 + kerb_width);

        let base_index = vertices.len() as u32;

        vertices.extend_from_slice(&[
            TrackVertex::new(
                right_inner1 + Vec3::Y * kerb_height,
                Vec3::Y,
                Vec2::ZERO,
                kerb_color,
            ),
            TrackVertex::new(
                right_outer1 + Vec3::Y * kerb_height,
                Vec3::Y,
                Vec2::ZERO,
                kerb_color,
            ),
            TrackVertex::new(
                right_inner2 + Vec3::Y * kerb_height,
                Vec3::Y,
                Vec2::ZERO,
                kerb_color,
            ),
            TrackVertex::new(
                right_outer2 + Vec3::Y * kerb_height,
                Vec3::Y,
                Vec2::ZERO,
                kerb_color,
            ),
        ]);

        indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index + 1,
            base_index + 3,
            base_index + 2,
        ]);
    }

    /// Calculate smooth vertex normals
    fn calculate_normals(vertices: &mut [TrackVertex], indices: &[u32]) {
        // Reset normals
        for v in vertices.iter_mut() {
            v.normal = [0.0, 0.0, 0.0];
        }

        // Accumulate face normals
        for triangle in indices.chunks(3) {
            let i0 = triangle[0] as usize;
            let i1 = triangle[1] as usize;
            let i2 = triangle[2] as usize;

            let v0 = Vec3::from_array(vertices[i0].position);
            let v1 = Vec3::from_array(vertices[i1].position);
            let v2 = Vec3::from_array(vertices[i2].position);

            // Calculate face normal
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(edge2);

            // Accumulate to vertex normals
            let n0 = Vec3::from_array(vertices[i0].normal);
            let n1 = Vec3::from_array(vertices[i1].normal);
            let n2 = Vec3::from_array(vertices[i2].normal);

            vertices[i0].normal = (n0 + normal).to_array();
            vertices[i1].normal = (n1 + normal).to_array();
            vertices[i2].normal = (n2 + normal).to_array();
        }

        // Normalize
        for v in vertices.iter_mut() {
            let normal = Vec3::from_array(v.normal);
            if normal.length_squared() > 0.0001 {
                v.normal = normal.normalize().to_array();
            } else {
                v.normal = Vec3::Y.to_array(); // Default to up if degenerate
            }
        }
    }

    /// Get color for surface type
    fn surface_color(surface: SurfaceType) -> [f32; 4] {
        match surface {
            SurfaceType::Track => [0.3, 0.3, 0.35, 1.0], // Dark gray asphalt
            SurfaceType::Grass => [0.2, 0.6, 0.2, 1.0],  // Green
            SurfaceType::Gravel => [0.7, 0.6, 0.4, 1.0], // Tan/brown
            SurfaceType::Kerb => [1.0, 0.2, 0.2, 1.0],   // Red
            SurfaceType::PitLane => [0.4, 0.4, 0.45, 1.0], // Lighter gray
            SurfaceType::Wall => [0.6, 0.6, 0.6, 1.0],   // Light gray
        }
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get index count
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_section(z: f32) -> TrackSection {
        TrackSection {
            position: Vec3::new(0.0, 0.0, z),
            length: 10.0,
            curvature: 0,
            height: 0,
            flags: 0,
            right_verge_width: 1,
            left_verge_width: 1,
            commands: Vec::new(),
            has_left_kerb: false,
            has_right_kerb: false,
            kerb_height: crate::data::KerbHeight::Low,
            pit_lane_entrance: false,
            pit_lane_exit: false,
            road_signs: false,
            road_sign_arrow: false,
            surface: SurfaceType::Track,
            width: 12.0,
            banking: 0.0,
            elevation: 0.0,
        }
    }

    #[test]
    fn test_track_mesh_creation() {
        // Create simple test track
        let track = Track {
            name: "Test Track".to_string(),
            length: 100.0,
            object_shapes: vec![],
            sections: vec![make_section(0.0), make_section(10.0)],
            racing_line: crate::data::RacingLine {
                displacement: 0,
                segments: vec![],
            },
            ai_behavior: crate::data::AIBehavior {
                aggression: 0.5,
                consistency: 0.8,
                car_setup: crate::data::CarSetup {
                    front_wing: 10,
                    rear_wing: 10,
                    gear_ratios: [5, 8, 11, 14, 17, 20],
                    brake_balance: 50,
                },
            },
            pit_lane: vec![],
            cameras: vec![],
            checksum: 0,
        };

        let mesh = TrackMesh::from_track(&track);

        // Should have vertices and indices
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.index_count() > 0);
        assert_eq!(mesh.index_count() % 3, 0); // Triangles

        // Bounds should be valid
        assert!(mesh.bounds_min.x < mesh.bounds_max.x);
        assert!(mesh.bounds_min.z < mesh.bounds_max.z);
    }

    #[test]
    fn test_surface_colors() {
        use crate::data::SurfaceType;

        let track_color = TrackMesh::surface_color(SurfaceType::Track);
        let grass_color = TrackMesh::surface_color(SurfaceType::Grass);

        // Track should be dark, grass should be green
        assert!(track_color[0] < 0.5); // Dark
        assert!(grass_color[1] > grass_color[0]); // Green channel dominant
    }
}
