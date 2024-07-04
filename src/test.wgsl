struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs(
    @builtin(vertex_index) in_vertex_index: u32,
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position.x, in.position.y, 0.0, 1.0);
    return out;
}

@fragment fn fs() -> @location(0) vec4f {
    return vec4f(0.2232279573168085, 0.6239603916750761, 0.29177064981753587, 1.0);
}