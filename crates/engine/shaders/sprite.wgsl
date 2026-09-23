// Vertex shader
@vertex
fn vs_main(@location(0) position: vec2<f32>, @location(1) color: vec4<f32>) -> VertexOutput {
    var out: VertexOutput;
    // NDC: map [0, viewport] to [-1, 1]
    // Note: Y is flipped (top = -1, bottom = 1)
    out.position = vec4<f32>(
        position.x * 2.0 / 800.0 - 1.0,
        -(position.y * 2.0 / 600.0 - 1.0),
        0.0,
        1.0
    );
    out.color = color;
    return out;
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
