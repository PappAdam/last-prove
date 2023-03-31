#version 450

layout(push_constant) uniform _push_const {
    float wh_ratio;
    float max_z;
    float min_z;
    float delta_time;
} push_const;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in vec3 normal;
layout(location = 3) in float wave_multip;
layout(binding = 0) uniform _view {
    mat4 view;
    mat4 rotation;
} view;

layout(location = 0) out vec3 fragColor;

#define r_ang PI / 2

void main()
{   
    vec3 light_source = normalize(vec3(2, 3, 1));

    vec4 new_pos = view.view * view.rotation *  vec4(pos.x, pos.yz, 1.0);

    float depth_z = (new_pos.z - push_const.min_z) / (push_const.max_z - push_const.min_z);

    float wave = sin(gl_VertexIndex / 36 * wave_multip * push_const.delta_time) / 10;

    gl_Position = vec4(new_pos.x * push_const.wh_ratio, new_pos.y + wave, depth_z, 1.);

    fragColor = vec3(color * dot((view.rotation * vec4(normal, 1.)).xyz, light_source) + color) / 2;
}
