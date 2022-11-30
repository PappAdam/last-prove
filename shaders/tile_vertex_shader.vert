#version 450

layout(push_constant) uniform PushConstant {
    uint tile_size;
    vec2 camera_position;
} pconsts;

layout(location = 0) in uint instance_x;
layout(location = 1) in uint instance_y;
layout(location = 2) in uint sampler_index;
layout(location = 3) in uint texture_layer;

layout(location = 0) out uint out_sampler_index;
layout(location = 1) out uint out_texture_layer;
layout(location = 2) out vec2 uv_coordinates;

void main() {
    gl_Position = vec4(
        (instance_x - instance_y) * pconsts.tile_size,
        (instance_x + instance_y) * pconsts.tile_size,
        0.0, 1.0
    );

    uint vertex_type = gl_VertexIndex % 4;
    vec2 uv_offsets = vec2(
        vertex_type / 2,
        vertex_type % 2
    );

    out_sampler_index = sampler_index;
    out_texture_layer = texture_layer;
    uv_coordinates = uv_offsets;
}