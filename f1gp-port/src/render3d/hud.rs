// HUD Renderer - Custom bitmap font overlay system
// Custom implementation to avoid egui/wgpu version conflicts

use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

/// Vertex for HUD text rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct HudVertex {
    pub position: [f32; 2],   // Screen space position
    pub tex_coords: [f32; 2], // Texture coordinates
    pub color: [f32; 4],      // Text color
}

impl HudVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<HudVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Simple bitmap font generator for ASCII characters
/// Creates a texture atlas with 96 printable ASCII characters (32-127)
fn generate_font_atlas() -> Vec<u8> {
    const CHAR_WIDTH: usize = 8;
    const CHAR_HEIGHT: usize = 16;
    const CHARS_PER_ROW: usize = 16;
    const ATLAS_WIDTH: usize = CHAR_WIDTH * CHARS_PER_ROW;
    const ATLAS_HEIGHT: usize = CHAR_HEIGHT * 6; // 6 rows for 96 chars

    let mut atlas = vec![0u8; ATLAS_WIDTH * ATLAS_HEIGHT * 4]; // RGBA

    // Create a simple font programmatically
    // Each character is 8x16 pixels
    for char_idx in 0..96 {
        let char_code = char_idx + 32; // Start from space (32)
        let row = char_idx / CHARS_PER_ROW;
        let col = char_idx % CHARS_PER_ROW;

        let start_x = col * CHAR_WIDTH;
        let start_y = row * CHAR_HEIGHT;

        // Simple pattern generation for each character
        for y in 0..CHAR_HEIGHT {
            for x in 0..CHAR_WIDTH {
                let pixel_x = start_x + x;
                let pixel_y = start_y + y;
                let pixel_idx = (pixel_y * ATLAS_WIDTH + pixel_x) * 4;

                // Generate a simple pattern for visible characters
                let is_pixel_on = get_char_pixel(char_code as u8, x, y);

                if is_pixel_on {
                    atlas[pixel_idx] = 255; // R
                    atlas[pixel_idx + 1] = 255; // G
                    atlas[pixel_idx + 2] = 255; // B
                    atlas[pixel_idx + 3] = 255; // A
                } else {
                    atlas[pixel_idx] = 0; // R
                    atlas[pixel_idx + 1] = 0; // G
                    atlas[pixel_idx + 2] = 0; // B
                    atlas[pixel_idx + 3] = 0; // A (transparent)
                }
            }
        }
    }

    atlas
}

/// Simple character pixel lookup
/// Returns true if the pixel at (x, y) should be lit for the given character
fn get_char_pixel(char_code: u8, x: usize, y: usize) -> bool {
    // Very simple font rendering - just draw a box for most chars for now
    // This is a placeholder; a real implementation would use actual font data

    match char_code {
        b' ' => false, // Space is always empty
        b'0'..=b'9' => {
            // Numbers: draw outline
            (x == 0 || x == 7 || y == 0 || y == 15) && (x < 7 && y < 15)
        }
        b'A'..=b'Z' | b'a'..=b'z' => {
            // Letters: draw a pattern
            (1..=6).contains(&x) && (2..=13).contains(&y) && (x + y).is_multiple_of(2)
        }
        b'.' => {
            // Period
            y >= 13 && (3..=4).contains(&x)
        }
        b':' => {
            // Colon
            ((5..=6).contains(&y) && (3..=4).contains(&x))
                || ((10..=11).contains(&y) && (3..=4).contains(&x))
        }
        b'/' => {
            // Forward slash
            (15 - y) == x * 2
        }
        b'|' => {
            // Pipe
            x == 4
        }
        _ => {
            // Default: small box
            (2..=5).contains(&x) && (6..=9).contains(&y)
        }
    }
}

/// HUD Renderer
pub struct HudRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    _font_texture: wgpu::Texture,
    _font_view: wgpu::TextureView,
    _font_sampler: wgpu::Sampler,
    bind_group: wgpu::BindGroup,
    num_indices: u32,
    screen_width: f32,
    screen_height: f32,
}

impl HudRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Result<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("HUD Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/hud.wgsl").into()),
        });

        // Create font atlas texture
        let font_atlas_data = generate_font_atlas();

        let texture_size = wgpu::Extent3d {
            width: 128, // 16 chars * 8 pixels
            height: 96, // 6 rows * 16 pixels
            depth_or_array_layers: 1,
        };

        let font_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("HUD Font Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            font_texture.as_image_copy(),
            &font_atlas_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(128 * 4),
                rows_per_image: Some(96),
            },
            texture_size,
        );

        let font_view = font_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let font_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("HUD Font Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("HUD Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("HUD Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&font_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&font_sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("HUD Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("HUD Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[HudVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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

        // Create initial buffers (will be updated each frame)
        let vertices: Vec<HudVertex> = vec![];
        let indices: Vec<u16> = vec![];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("HUD Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("HUD Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        Ok(Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            _font_texture: font_texture,
            _font_view: font_view,
            _font_sampler: font_sampler,
            bind_group,
            num_indices: 0,
            screen_width: config.width as f32,
            screen_height: config.height as f32,
        })
    }

    /// Resize the HUD renderer when window size changes
    pub fn resize(&mut self, width: u32, height: u32) {
        self.screen_width = width as f32;
        self.screen_height = height as f32;
    }

    /// Build text geometry for rendering
    /// Returns (vertices, indices) for the text
    fn build_text_geometry(
        &self,
        text: &str,
        x: f32,
        y: f32,
        scale: f32,
        color: [f32; 4],
    ) -> (Vec<HudVertex>, Vec<u16>) {
        const CHAR_WIDTH: f32 = 8.0;
        const CHAR_HEIGHT: f32 = 16.0;
        const _CHARS_PER_ROW: f32 = 16.0;
        const ATLAS_WIDTH: f32 = 128.0;
        const ATLAS_HEIGHT: f32 = 96.0;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let char_screen_width = CHAR_WIDTH * scale;
        let char_screen_height = CHAR_HEIGHT * scale;

        for (i, ch) in text.chars().enumerate() {
            let char_code = ch as u8;
            if !(32..128).contains(&char_code) {
                continue; // Skip non-printable characters
            }

            let char_idx = (char_code - 32) as usize;
            let tex_col = (char_idx % 16) as f32;
            let tex_row = (char_idx / 16) as f32;

            // Texture coordinates in atlas
            let tex_x = tex_col * CHAR_WIDTH / ATLAS_WIDTH;
            let tex_y = tex_row * CHAR_HEIGHT / ATLAS_HEIGHT;
            let tex_w = CHAR_WIDTH / ATLAS_WIDTH;
            let tex_h = CHAR_HEIGHT / ATLAS_HEIGHT;

            // Screen position (normalized to -1..1)
            let screen_x = (x + i as f32 * char_screen_width) / self.screen_width * 2.0 - 1.0;
            let screen_y = 1.0 - (y / self.screen_height * 2.0);
            let screen_w = char_screen_width / self.screen_width * 2.0;
            let screen_h = char_screen_height / self.screen_height * 2.0;

            let base_index = vertices.len() as u16;

            // Four vertices for this character quad
            vertices.push(HudVertex {
                position: [screen_x, screen_y],
                tex_coords: [tex_x, tex_y],
                color,
            });
            vertices.push(HudVertex {
                position: [screen_x + screen_w, screen_y],
                tex_coords: [tex_x + tex_w, tex_y],
                color,
            });
            vertices.push(HudVertex {
                position: [screen_x + screen_w, screen_y - screen_h],
                tex_coords: [tex_x + tex_w, tex_y + tex_h],
                color,
            });
            vertices.push(HudVertex {
                position: [screen_x, screen_y - screen_h],
                tex_coords: [tex_x, tex_y + tex_h],
                color,
            });

            // Two triangles
            indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index,
                base_index + 2,
                base_index + 3,
            ]);
        }

        (vertices, indices)
    }

    /// Render HUD text overlay
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        lines: &[(String, f32, f32, f32, [f32; 4])], // (text, x, y, scale, color)
    ) {
        // Build geometry for all text lines
        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();

        for (text, x, y, scale, color) in lines {
            let (mut vertices, indices) = self.build_text_geometry(text, *x, *y, *scale, *color);

            // Offset indices by current vertex count
            let base_index = all_vertices.len() as u16;
            all_indices.extend(indices.iter().map(|i| i + base_index));
            all_vertices.append(&mut vertices);
        }

        if all_indices.is_empty() {
            return; // Nothing to render
        }

        // Update buffers
        self.num_indices = all_indices.len() as u32;

        // Recreate buffers if size changed
        if !all_vertices.is_empty() {
            self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("HUD Vertex Buffer"),
                contents: bytemuck::cast_slice(&all_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("HUD Index Buffer"),
                contents: bytemuck::cast_slice(&all_indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        }

        // Render pass
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("HUD Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Don't clear - render on top
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
