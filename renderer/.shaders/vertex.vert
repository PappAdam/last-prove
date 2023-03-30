#version 450

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in vec3 normal;
layout(binding = 0) uniform _view {
    mat4 view;
    mat4 rotation;
} view;

layout(location = 0) out vec3 fragColor;

#define r_ang PI / 2

void main()
{   
    vec3 light_source = (vec4(normalize(vec3(2, 3, 1)), 1.) * view.view).xyz;

    vec4 new_pos = view.view * view.rotation *  vec4(pos, 1.0);

    float depth_z = (new_pos.z + 3.0) / 6.0;

    gl_Position = vec4(new_pos.xy, depth_z, 2.);

    fragColor = vec3(color * dot((view.rotation * vec4(normal, 1.)).xyz, light_source) + vec3(1., 1., 1.)) / 2;
}