#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(1) @binding(0) var read_texture: texture_2d<f32>;
@group(1) @binding(1) var read_texture_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(read_texture, read_texture_sampler, mesh.uv);
}