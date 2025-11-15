// Renderer3D - wgpu-based 3D renderer
// Stage 6.1: Basic 3D Setup
// Stage 6.2: Track Rendering Integration

use anyhow::Result;
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

use crate::game::GameState;
use crate::data::Track;
use super::camera3d::Camera3D;
use super::track_mesh::{TrackMesh, TrackVertex};
use super::car_model::{CarModel, CarVertex};

/// Vertex for basic 3D rendering (test triangle)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Uniforms for camera transformation
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniforms {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniforms {
    fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update(&mut self, camera: &Camera3D) {
        let view_proj = camera.projection_matrix() * camera.view_matrix();
        self.view_proj = view_proj.to_cols_array_2d();
    }
}

/// Uniforms for lighting
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct LightUniforms {
    direction: [f32; 3],
    _padding: f32,
    color: [f32; 3],
    ambient: f32,
}

impl LightUniforms {
    fn new() -> Self {
        Self {
            direction: [0.5, -1.0, 0.3],  // Sun from top-right
            _padding: 0.0,
            color: [1.0, 1.0, 0.95],      // Slightly warm white
            ambient: 0.3,                  // 30% ambient light
        }
    }
}

/// Uniforms for model transformation (per-car)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct ModelUniforms {
    model: [[f32; 4]; 4],
}

impl ModelUniforms {
    fn new() -> Self {
        Self {
            model: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update(&mut self, transform: Mat4) {
        self.model = transform.to_cols_array_2d();
    }
}

/// 3D Renderer using wgpu
pub struct Renderer3D {
    pub camera: Camera3D,
    camera_uniforms: CameraUniforms,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    // Lighting
    light_uniforms: LightUniforms,
    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,

    // Depth buffer
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,

    // Track rendering
    track_pipeline: wgpu::RenderPipeline,
    track_vertex_buffer: Option<wgpu::Buffer>,
    track_index_buffer: Option<wgpu::Buffer>,
    track_index_count: u32,

    // Test triangle (for basic testing)
    basic_pipeline: wgpu::RenderPipeline,
    test_vertex_buffer: wgpu::Buffer,
    test_vertex_count: u32,

    // Car rendering
    car_pipeline: wgpu::RenderPipeline,
    car_model: CarModel,
    car_vertex_buffer: wgpu::Buffer,
    car_index_buffer: wgpu::Buffer,
    car_index_count: u32,
    model_buffer: wgpu::Buffer,
    model_bind_group_layout: wgpu::BindGroupLayout,
}

impl Renderer3D {
    /// Create new 3D renderer with track rendering support
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Result<Self> {
        let aspect_ratio = config.width as f32 / config.height as f32;
        let camera = Camera3D::new(aspect_ratio);

        // Create camera uniforms
        let mut camera_uniforms = CameraUniforms::new();
        camera_uniforms.update(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create camera bind group layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Create light uniforms
        let light_uniforms = LightUniforms::new();
        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::cast_slice(&[light_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create light bind group layout
        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Light Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Light Bind Group"),
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
        });

        // Create depth texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Load track shader
        let track_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Track Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/track.wgsl").into()),
        });

        // Create track render pipeline
        let track_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Track Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                push_constant_ranges: &[],
            });

        let track_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Track Pipeline"),
            layout: Some(&track_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &track_shader,
                entry_point: Some("vs_main"),
                buffers: &[TrackVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &track_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Load basic shader for test triangle
        let basic_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Basic Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/basic.wgsl").into()),
        });

        // Create basic render pipeline (no depth test)
        let basic_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Basic Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let basic_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Basic Pipeline"),
            layout: Some(&basic_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &basic_shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &basic_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Create test triangle
        let test_vertices = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ];

        let test_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Test Vertex Buffer"),
            contents: bytemuck::cast_slice(&test_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create model bind group layout (Group 2 for car shader)
        let model_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Model Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create model uniform buffer
        let model_uniforms = ModelUniforms::new();
        let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: bytemuck::cast_slice(&[model_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Load car shader
        let car_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Car Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/car.wgsl").into()),
        });

        // Create car render pipeline
        let car_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Car Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &light_bind_group_layout,
                    &model_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let car_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Car Pipeline"),
            layout: Some(&car_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &car_shader,
                entry_point: Some("vs_main"),
                buffers: &[CarVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &car_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Create default car model (red for now, will be replaced per-team)
        let car_model = CarModel::create_f1_car([1.0, 0.0, 0.0, 1.0]);

        // Create car vertex buffer
        let car_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Car Vertex Buffer"),
            contents: bytemuck::cast_slice(&car_model.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create car index buffer
        let car_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Car Index Buffer"),
            contents: bytemuck::cast_slice(&car_model.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let car_index_count = car_model.indices.len() as u32;

        Ok(Self {
            camera,
            camera_uniforms,
            camera_buffer,
            camera_bind_group,
            light_uniforms,
            light_buffer,
            light_bind_group,
            depth_texture,
            depth_view,
            track_pipeline,
            track_vertex_buffer: None,
            track_index_buffer: None,
            track_index_count: 0,
            basic_pipeline,
            test_vertex_buffer,
            test_vertex_count: test_vertices.len() as u32,
            car_pipeline,
            car_model,
            car_vertex_buffer,
            car_index_buffer,
            car_index_count,
            model_buffer,
            model_bind_group_layout,
        })
    }

    /// Load track mesh into GPU buffers
    pub fn load_track(&mut self, device: &wgpu::Device, track: &Track) {
        let mesh = TrackMesh::from_track(track);

        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Track Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Track Index Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.track_vertex_buffer = Some(vertex_buffer);
        self.track_index_buffer = Some(index_buffer);
        self.track_index_count = mesh.indices.len() as u32;
    }

    /// Update camera from game state
    pub fn update(&mut self, game_state: &GameState, queue: &wgpu::Queue) {
        self.camera.update_from_car(game_state.player_car(), 0.016);
        self.camera_uniforms.update(&self.camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniforms]),
        );
    }

    /// Render a frame
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) -> Result<()> {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("3D Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.53,
                        g: 0.81,
                        b: 0.92,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Render track if loaded
        if let (Some(vertex_buffer), Some(index_buffer)) =
            (&self.track_vertex_buffer, &self.track_index_buffer)
        {
            render_pass.set_pipeline(&self.track_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.light_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.track_index_count, 0, 0..1);
        }

        drop(render_pass);

        Ok(())
    }

    /// Render cars from game state
    pub fn render_cars(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        game_state: &GameState,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        // Create model bind group for rendering
        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Model Bind Group"),
            layout: &self.model_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.model_buffer.as_entire_binding(),
            }],
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Car Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Don't clear, we already rendered track
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Keep existing depth
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.car_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(1, &self.light_bind_group, &[]);
        render_pass.set_bind_group(2, &model_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.car_vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.car_index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        // Render player car
        let player_car = game_state.player_car();
        let transform = CarModel::get_transform_matrix(player_car);
        let mut model_uniforms = ModelUniforms::new();
        model_uniforms.update(transform);
        queue.write_buffer(&self.model_buffer, 0, bytemuck::cast_slice(&[model_uniforms]));
        render_pass.draw_indexed(0..self.car_index_count, 0, 0..1);

        // TODO: Render AI cars when we have access to them

        drop(render_pass);

        Ok(())
    }

    /// Handle window resize
    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let aspect_ratio = width as f32 / height as f32;
            self.camera.set_aspect_ratio(aspect_ratio);

            // Recreate depth texture
            self.depth_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

            self.depth_view = self.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }
}
