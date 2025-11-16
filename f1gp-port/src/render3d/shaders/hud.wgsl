// HUD Shader - Simple 2D text overlay
// Renders bitmap font characters with alpha blending

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@group(0) @binding(0)
var t_font: texture_2d<f32>;
@group(0) @binding(1)
var s_font: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 0.0, 1.0);
    output.tex_coords = input.tex_coords;
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let tex_sample = textureSample(t_font, s_font, input.tex_coords);

    // Use texture alpha channel for transparency
    // Multiply by vertex color for tinting
    return vec4<f32>(input.color.rgb, input.color.a * tex_sample.a);
}
