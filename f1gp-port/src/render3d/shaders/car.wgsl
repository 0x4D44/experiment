// Car Rendering Shader - Stage 6.3 & 6.5
// Renders 3D car models with lighting, specular highlights, and fog

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

struct ModelUniforms {
    model: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

@group(1) @binding(0)
var<uniform> light: LightUniforms;

@group(2) @binding(0)
var<uniform> model: ModelUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position by model matrix, then by view-projection
    let world_position = model.model * vec4<f32>(in.position, 1.0);
    out.clip_position = camera.view_proj * world_position;
    out.world_position = world_position.xyz;

    // Transform normal by model matrix (should use inverse transpose for non-uniform scaling)
    // For now, simple rotation is fine since we don't have scaling
    let world_normal = (model.model * vec4<f32>(in.normal, 0.0)).xyz;
    out.world_normal = normalize(world_normal);

    out.color = in.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Normalize the normal (interpolation can denormalize it)
    let normal = normalize(in.world_normal);

    // Light direction (should be normalized in uniform)
    let light_dir = normalize(-light.direction);

    // Lambertian diffuse lighting
    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let diffuse = (1.0 - light.ambient) * n_dot_l;

    // Blinn-Phong specular lighting
    let view_dir = normalize(camera.camera_pos - in.world_position);
    let half_dir = normalize(light_dir + view_dir);
    let n_dot_h = max(dot(normal, half_dir), 0.0);
    let specular_strength = 0.3; // Car paint has some shine
    let shininess = 32.0; // Moderate shininess
    let specular = specular_strength * pow(n_dot_h, shininess);

    // Combine lighting components
    let lighting = light.ambient + diffuse + specular;

    // Apply lighting to car color
    var lit_color = in.color.rgb * light.color * lighting;

    // Fog effect (distance-based) - matches enhanced skybox horizon
    let fog_color = vec3<f32>(0.75, 0.85, 0.92); // Lighter horizon color
    let distance = length(camera.camera_pos - in.world_position);
    let fog_start = 80.0;  // Start fog sooner
    let fog_end = 300.0;   // Closer fog for more atmospheric depth
    let fog_factor = clamp((distance - fog_start) / (fog_end - fog_start), 0.0, 1.0);
    lit_color = mix(lit_color, fog_color, fog_factor);

    return vec4<f32>(lit_color, in.color.a);
}
