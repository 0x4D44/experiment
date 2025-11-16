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

    // Enhanced sky gradient with atmospheric perspective
    let sky_top = vec3<f32>(0.2, 0.5, 0.9);      // Deeper blue at zenith
    let sky_mid = vec3<f32>(0.4, 0.7, 0.95);     // Mid-sky blue
    let sky_horizon = vec3<f32>(0.75, 0.85, 0.92); // Lighter blue-white at horizon
    let horizon_glow = vec3<f32>(0.95, 0.92, 0.85); // Warm glow near horizon

    // Vertical blend factor (0 = top, 1 = bottom)
    let vertical_t = (1.0 - dir.y) * 0.5;

    // Enhanced horizon glow (exponential falloff)
    let horizon_factor = pow(1.0 - abs(dir.y), 3.0);

    // Blend sky colors
    var sky_color: vec3<f32>;
    if (dir.y > 0.0) {
        // Upper hemisphere: top to mid
        sky_color = mix(sky_top, sky_mid, vertical_t * 2.0);
    } else {
        // Lower hemisphere: mid to horizon
        sky_color = mix(sky_mid, sky_horizon, (vertical_t - 0.5) * 2.0);
    }

    // Add horizon glow
    sky_color = mix(sky_color, horizon_glow, horizon_factor * 0.3);

    return vec4<f32>(sky_color, 1.0);
}
