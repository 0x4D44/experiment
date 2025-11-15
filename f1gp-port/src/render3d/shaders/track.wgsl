// Track Rendering Shader - Stage 6.2 & 6.5
// Renders track surface with lighting and fog

struct CameraUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
    _padding: f32,
}

struct LightUniforms {
    direction: vec3<f32>,
    _padding: f32,
    color: vec3<f32>,
    ambient: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

@group(1) @binding(0)
var<uniform> light: LightUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.world_position = in.position;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    out.normal = in.normal;
    out.uv = in.uv;
    out.color = in.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Normalize the normal (interpolation can denormalize it)
    let normal = normalize(in.normal);

    // Light direction (should be normalized in uniform)
    let light_dir = normalize(-light.direction);

    // Lambertian diffuse lighting
    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let diffuse = (1.0 - light.ambient) * n_dot_l;

    // Subtle specular for wet/smooth asphalt
    let view_dir = normalize(camera.camera_pos - in.world_position);
    let half_dir = normalize(light_dir + view_dir);
    let n_dot_h = max(dot(normal, half_dir), 0.0);
    let specular_strength = 0.05; // Very subtle for asphalt
    let shininess = 8.0; // Low shininess (rough surface)
    let specular = specular_strength * pow(n_dot_h, shininess);

    // Combine lighting components
    let lighting = light.ambient + diffuse + specular;

    // Apply lighting to surface color
    var lit_color = in.color.rgb * light.color * lighting;

    // Fog effect (distance-based)
    let fog_color = vec3<f32>(0.53, 0.81, 0.92); // Sky blue
    let distance = length(camera.camera_pos - in.world_position);
    let fog_start = 100.0;
    let fog_end = 500.0;
    let fog_factor = clamp((distance - fog_start) / (fog_end - fog_start), 0.0, 1.0);
    lit_color = mix(lit_color, fog_color, fog_factor);

    return vec4<f32>(lit_color, in.color.a);
}
