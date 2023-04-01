#version 450

layout(push_constant) uniform _push_const {
    float wh_ratio;
    float max_z;
    float min_z;
    float time;
} push_const;

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
    vec3 light_source = normalize(vec3(0, sin(push_const.time / 2) * 10, cos(push_const.time / 2) * 10));
    float light_strenght = (sin(push_const.time / 2) + 1) / 3 + 0.2;
    vec3 light_color = vec3(1., 1., 1.);

    vec4 new_pos = view.view * view.rotation *  vec4(pos.x, pos.yz, 1.0);

    float depth_z = (new_pos.z - push_const.min_z) / (push_const.max_z - push_const.min_z);

    if (color.b > color.g) {
        float wave = sin(push_const.time + -pos.y * pos.y * 70) / 30;
        vec3 new_color = color - vec3(wave * 4, wave * 4, wave * 4);
        fragColor = vec3(color * dot((view.rotation * vec4(normal, 1.)).xyz, light_source) + new_color) / vec3(2.4, 2, 1.8) * light_strenght * light_color;
        new_pos.y += wave;
    }
    else {
        fragColor = vec3(color * dot((view.rotation * vec4(normal, 1.)).xyz, light_source) + color) * light_strenght * light_color;
    }

    gl_Position = vec4(new_pos.x * push_const.wh_ratio, new_pos.y, depth_z, 3.);
}
