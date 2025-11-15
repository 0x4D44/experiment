// Skybox Shader - Stage 6.5
// Renders a gradient sky background

struct CameraUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
    _padding: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) direction: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Position skybox at camera position (always centered on camera)
    let world_pos = in.position + camera.camera_pos;
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);

    // Make sure skybox is always at far plane (w = z)
    out.clip_position.z = out.clip_position.w;

    // Direction for gradient calculation
    out.direction = in.position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dir = normalize(in.direction);

    // Sky gradient: blue at top, lighter blue at horizon
    let sky_top = vec3<f32>(0.3, 0.6, 0.95);     // Deeper blue
    let sky_horizon = vec3<f32>(0.7, 0.85, 0.95); // Lighter blue

    // Blend based on vertical direction (-1 to 1)
    // At horizon (y=0), factor = 0.5; at top (y=1), factor = 0; at bottom (y=-1), factor = 1
    let t = (1.0 - dir.y) * 0.5;
    let sky_color = mix(sky_top, sky_horizon, t);

    return vec4<f32>(sky_color, 1.0);
}
