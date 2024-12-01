@group(0) @binding(0) var<uniform> time: f32;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let x = f32(i32(vertex_index) - 1);
    let y = f32(i32(vertex_index & 1u) * 2 - 1);

    // A spinning triangle is better than a static triangle
    let cos_theta = cos(time / 2);
    let sin_theta = sin(time / 2);
    let rotation_matrix = mat2x2<f32>(
        cos_theta, -sin_theta,
        sin_theta, cos_theta
    );
    var pos = vec2<f32>(x, y);
    let scale = 0.75 + sin(time / 5) * 0.25;
    pos = rotation_matrix * pos * scale;

    // And slowly rotate through colors to make it visually interesting
    let offset = f32(vertex_index) * 0.5;
    let r = 0.5 + 0.5 * sin(time + offset);
    let g = 0.5 + 0.5 * sin(time * 0.8 + offset + 4.0);
    let b = 0.5 + 0.5 * sin(time * 1.2 + offset + 8.0);
    let color = vec4<f32>(r, g, b, 1.0);

    var result: VertexOutput;
    result.position = vec4<f32>(rotation_matrix * pos * scale, 0.0, 1.0);
    result.color = color;
    return result;
}

@fragment fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}