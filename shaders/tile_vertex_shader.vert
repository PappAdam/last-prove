#version 450

layout(push_constant) uniform PushConstant {
    uint tile_size;
    vec2 camera_position;
} pconsts;

layout(location = 0) in uint coordinates; // First 16 representing x last 16 representing y.
layout(location = 1) in uint sampler_and_layer;

layout(location = 0) out uint out_sampler_and_layer;
layout(location = 1) out vec2 uv_coordinates;

void main() {
    gl_Position = vec4(
        //(coordinates.x - coordinates.y) * pconsts.tile_size,
        //(coordinates.x + coordinates.y) * pconsts.tile_size,
        0, 0,
        0.0, 1.0
    );

    uint vertex_type = gl_VertexIndex % 4;
    vec2 uv_offsets = vec2(
        vertex_type / 2,
        vertex_type % 2
    );

    out_sampler_and_layer = sampler_and_layer;
    uv_coordinates = uv_offsets;
}