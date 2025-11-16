// Car Model Generator - Stage 6.3
// Creates 3D car models for rendering

use glam::{Mat4, Quat, Vec3};
use bytemuck::{Pod, Zeroable};
use crate::physics::CarPhysics;

/// Vertex for car rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CarVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}

impl CarVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CarVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    /// Create vertex from Vec3
    pub fn new(position: Vec3, normal: Vec3, color: [f32; 4]) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
            color,
        }
    }
}

/// Simple box-based car model (like original F1GP)
pub struct CarModel {
    pub vertices: Vec<CarVertex>,
    pub indices: Vec<u32>,
}

impl CarModel {
    /// Create a simple box-based F1 car model
    pub fn create_f1_car(team_color: [f32; 4]) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // F1 car dimensions (approximate)
        let length = 4.5;   // 4.5m long
        let width = 2.0;    // 2m wide
        let height = 1.0;   // 1m tall
        let half_len = length * 0.5;
        let half_width = width * 0.5;

        // Main body (box)
        Self::add_box(
            &mut vertices,
            &mut indices,
            Vec3::new(0.0, height * 0.5, 0.0),
            Vec3::new(width, height, length),
            team_color,
        );

        // Front wing (low, wide)
        Self::add_box(
            &mut vertices,
            &mut indices,
            Vec3::new(0.0, 0.1, -half_len - 0.3),
            Vec3::new(width * 1.2, 0.1, 0.5),
            [0.1, 0.1, 0.1, 1.0], // Black
        );

        // Rear wing (high, wide)
        Self::add_box(
            &mut vertices,
            &mut indices,
            Vec3::new(0.0, height + 0.3, half_len - 0.2),
            Vec3::new(width * 1.1, 0.1, 0.4),
            [0.1, 0.1, 0.1, 1.0], // Black
        );

        // Wheels (4 cylinders approximated as boxes)
        let wheel_radius = 0.35;
        let wheel_width = 0.3;
        let wheel_color = [0.05, 0.05, 0.05, 1.0]; // Very dark gray

        // Front left wheel
        Self::add_box(
            &mut vertices,
            &mut indices,
            Vec3::new(-half_width - wheel_width * 0.5, wheel_radius, -half_len + 0.8),
            Vec3::new(wheel_width, wheel_radius * 2.0, wheel_radius * 2.0),
            wheel_color,
        );

        // Front right wheel
        Self::add_box(
            &mut vertices,
            &mut indices,
            Vec3::new(half_width + wheel_width * 0.5, wheel_radius, -half_len + 0.8),
            Vec3::new(wheel_width, wheel_radius * 2.0, wheel_radius * 2.0),
            wheel_color,
        );

        // Rear left wheel
        Self::add_box(
            &mut vertices,
            &mut indices,
            Vec3::new(-half_width - wheel_width * 0.5, wheel_radius, half_len - 0.8),
            Vec3::new(wheel_width, wheel_radius * 2.0, wheel_radius * 2.0),
            wheel_color,
        );

        // Rear right wheel
        Self::add_box(
            &mut vertices,
            &mut indices,
            Vec3::new(half_width + wheel_width * 0.5, wheel_radius, half_len - 0.8),
            Vec3::new(wheel_width, wheel_radius * 2.0, wheel_radius * 2.0),
            wheel_color,
        );

        // Cockpit (darker color on top)
        let mut cockpit_color = team_color;
        cockpit_color[0] *= 0.5;
        cockpit_color[1] *= 0.5;
        cockpit_color[2] *= 0.5;

        Self::add_box(
            &mut vertices,
            &mut indices,
            Vec3::new(0.0, height + 0.2, 0.0),
            Vec3::new(width * 0.6, 0.4, length * 0.3),
            cockpit_color,
        );

        Self {
            vertices,
            indices,
        }
    }

    /// Add a box to the mesh
    fn add_box(
        vertices: &mut Vec<CarVertex>,
        indices: &mut Vec<u32>,
        center: Vec3,
        size: Vec3,
        color: [f32; 4],
    ) {
        let base_index = vertices.len() as u32;
        let half_size = size * 0.5;

        // 8 vertices of the box
        let positions = [
            center + Vec3::new(-half_size.x, -half_size.y, -half_size.z), // 0: left-bottom-front
            center + Vec3::new(half_size.x, -half_size.y, -half_size.z),  // 1: right-bottom-front
            center + Vec3::new(half_size.x, half_size.y, -half_size.z),   // 2: right-top-front
            center + Vec3::new(-half_size.x, half_size.y, -half_size.z),  // 3: left-top-front
            center + Vec3::new(-half_size.x, -half_size.y, half_size.z),  // 4: left-bottom-back
            center + Vec3::new(half_size.x, -half_size.y, half_size.z),   // 5: right-bottom-back
            center + Vec3::new(half_size.x, half_size.y, half_size.z),    // 6: right-top-back
            center + Vec3::new(-half_size.x, half_size.y, half_size.z),   // 7: left-top-back
        ];

        // 6 faces, each with a normal
        let faces = [
            // Front face (negative Z)
            ([0, 1, 2, 3], Vec3::new(0.0, 0.0, -1.0)),
            // Back face (positive Z)
            ([5, 4, 7, 6], Vec3::new(0.0, 0.0, 1.0)),
            // Left face (negative X)
            ([4, 0, 3, 7], Vec3::new(-1.0, 0.0, 0.0)),
            // Right face (positive X)
            ([1, 5, 6, 2], Vec3::new(1.0, 0.0, 0.0)),
            // Bottom face (negative Y)
            ([4, 5, 1, 0], Vec3::new(0.0, -1.0, 0.0)),
            // Top face (positive Y)
            ([3, 2, 6, 7], Vec3::new(0.0, 1.0, 0.0)),
        ];

        for (face_indices, normal) in &faces {
            let start = vertices.len() as u32;

            // Add 4 vertices for this face
            for &idx in face_indices {
                vertices.push(CarVertex::new(positions[idx], *normal, color));
            }

            // Add 2 triangles for this face
            indices.extend_from_slice(&[
                start,
                start + 1,
                start + 2,
                start,
                start + 2,
                start + 3,
            ]);
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

    /// Get transform matrix for a car
    pub fn get_transform_matrix(car: &CarPhysics) -> Mat4 {
        // Translation
        let translation = Mat4::from_translation(car.body.position);

        // Rotation from quaternion
        let rotation = Mat4::from_quat(car.body.orientation);

        // Combine (translation * rotation)
        translation * rotation
    }
}

/// LOD (Level of Detail) level for car rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LODLevel {
    High,    // < 50m
    Medium,  // 50-200m
    Low,     // 200-500m
    Billboard, // > 500m (not implemented yet)
}

impl LODLevel {
    /// Determine LOD level based on distance
    pub fn from_distance(distance: f32) -> Self {
        if distance < 50.0 {
            LODLevel::High
        } else if distance < 200.0 {
            LODLevel::Medium
        } else if distance < 500.0 {
            LODLevel::Low
        } else {
            LODLevel::Billboard
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_model_creation() {
        let team_color = [1.0, 0.0, 0.0, 1.0]; // Red
        let model = CarModel::create_f1_car(team_color);

        // Should have vertices and indices
        assert!(model.vertex_count() > 0);
        assert!(model.index_count() > 0);
        assert_eq!(model.index_count() % 3, 0); // Triangles
    }

    #[test]
    fn test_lod_levels() {
        assert_eq!(LODLevel::from_distance(25.0), LODLevel::High);
        assert_eq!(LODLevel::from_distance(100.0), LODLevel::Medium);
        assert_eq!(LODLevel::from_distance(300.0), LODLevel::Low);
        assert_eq!(LODLevel::from_distance(600.0), LODLevel::Billboard);
    }

    #[test]
    fn test_transform_matrix() {
        use crate::data::CarDatabase;
        use crate::physics::BodyId;

        let car_database = CarDatabase::create_sample();
        let car_spec = car_database.cars().next().unwrap().clone();
        let car = CarPhysics::new(BodyId(0), car_spec, Vec3::new(10.0, 1.0, 20.0));

        let transform = CarModel::get_transform_matrix(&car);

        // Transform should include translation
        let origin = transform.transform_point3(Vec3::ZERO);
        assert!((origin - car.body.position).length() < 0.01);
    }
}
