#version 450

layout(push_constant) uniform Camera {
    uint tile_size;
    vec2 coordinates;
    vec2 size;
} camera;

layout(location = 0) in vec2 coordinates; // First 16 representing x last 16 representing y.
//layout(location = 1) in uint sampler_and_layer;

//layout(location = 0) out uint out_sampler_and_layer;
layout(location = 1) out vec2 uv_coordinates;

void main() {
    uint vertex_type = gl_VertexIndex % 4;
    vec2 offsets = vec2(
       vertex_type / 2,
       vertex_type % 2
    );
    vec2 relative_tile_size = vec2(
       camera.tile_size / camera.size.x,
       camera.tile_size / camera.size.y
    );

    vec2 instance_coordinates = vec2(
       coordinates.x - camera.coordinates.x - 1,
       coordinates.y - camera.coordinates.y
    );

    vec2 vertex_position = vec2(
       (instance_coordinates.x - instance_coordinates.y) * relative_tile_size.x / 2 + offsets.x * relative_tile_size.x,
       (instance_coordinates.x + instance_coordinates.y) * relative_tile_size.y / 4 + offsets.y * relative_tile_size.y
       );

    gl_Position = vec4(vertex_position, 0.0, 1.0);


    //out_sampler_and_layer = sampler_and_layer;
    uv_coordinates = offsets;
}