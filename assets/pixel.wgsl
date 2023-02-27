#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct CustomMaterial {
    color: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> material: CustomMaterial;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var position = vertex.position;
    let round_to = 0.07;

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.));

    out.clip_position.x = round(out.clip_position.x / round_to) * round_to;
    out.clip_position.y = round(out.clip_position.y / round_to) * round_to;
    out.clip_position.z = round(out.clip_position.z / round_to) * round_to;

    out.blend_color = vertex.blend_color + vec4<f32>(position, 0.0);
    return out;
}

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    let lambda = abs(sin(globals.time)) * 0.7;
    return mix(input.blend_color, material.color, lambda * lambda * lambda);
}
