#version 450

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(binding = 0) uniform _view {
    mat4 view;
} view;

layout(location = 0) out vec3 fragColor;

#define r_ang PI / 2

void main()
{   
    vec4 new_pos = view.view * vec4(pos, 1.0);

    float depth_z = (new_pos.z + 3.0) / 6.0;

    gl_Position = vec4(new_pos.xy, depth_z, 1.0);
    fragColor = vec3(depth_z * depth_z, depth_z * depth_z, depth_z * depth_z);
}
