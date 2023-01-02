#version 450

layout(location = 0) in vec3 screen_position;
layout(location = 1) in vec2 object_size;
layout(location = 2) in uint texture_layer;

layout(location = 0) out uint out_texture_layer;
layout(location = 1) out vec2 uv_coordinates;

void main() {
    uint vertex_type = gl_VertexIndex % 4;

    vec3 offsets = vec3(
       (vertex_type / 2) * object_size.x,
       (vertex_type % 2) * object_size.y,
       0
    );

    gl_Position = vec4(screen_position + offsets, 1.0);
}