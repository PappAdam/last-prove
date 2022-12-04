#version 450

layout(location = 0) out vec4 f_color;

//layout(location = 0) flat in uint sampler_and_layer;
layout(location = 1) in vec2 uv_coordinates;

layout(set = 0, binding = 0) uniform sampler2DArray[] samplers;

void main() {
    f_color = texture(samplers[0], vec3(uv_coordinates, 0));
}