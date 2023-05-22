#version 450

layout(push_constant) uniform _push_const {
    float wh_ratio;
    float max_z;
    float min_z;
} push_const;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in vec3 normal;
layout(binding = 0) uniform _view {
    mat4 view;
    mat4 rotation;
} view;

layout(binding = 1) uniform _model {
    mat4 transform;
} model;

layout(location = 0) out vec3 fragColor;

void main()
{   
    mat4 model_view = view.view * model.transform;

    vec3 light_source = normalize(vec3(0.3, -1, 0));
    //Direction of the light

    vec4 new_pos = model_view * vec4(pos, 1.);

    float depth_z = (new_pos.z - push_const.min_z) / (push_const.max_z - push_const.min_z);

    gl_Position = vec4(new_pos.x * push_const.wh_ratio, new_pos.y, depth_z, 1.);

    // fragColor = color * dot(normalize(vec3(model_view * vec4(normal, 0.0))), normalize(vec3(view.view * vec4(light_source, 0.0))));
    // fragColor = vec3(model.transform * vec4(normal, 0.0));
    fragColor = color;
}
