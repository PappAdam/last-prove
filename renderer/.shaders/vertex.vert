#version 450

layout(push_constant) uniform _push_const {
    float wh_ratio;
    float min_z;
    float max_z;
    vec3 sun_direction;
    vec3 sun_color;
} push_const;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in vec3 normal;
layout(binding = 0) uniform _view {
    mat4 view;
} view;

layout(binding = 1) uniform _model {
    mat4 transform;
} model;

layout(location = 0) out vec3 fragColor;

void main()
{   
    mat4 model_view = view.view * model.transform;

    // Position calculation
    vec4 new_pos = model_view * vec4(pos, 1.);
    float depth_z = (new_pos.z - push_const.min_z) / (push_const.max_z - push_const.min_z);
    gl_Position = vec4(new_pos.x * push_const.wh_ratio, new_pos.y, depth_z, 1.);

    // Color/light calculation
    vec4 sun_direction = vec4(push_const.sun_direction, 1.0);
    // vec4 sun_direction = vec4(0.0, 0.0, 0.0, 1.0);
    vec4 sun_color = vec4(push_const.sun_color, 1.0);
    // vec4 sun_color = vec4(1.0, 0.0, 0.0, 1.0);
    vec4 sun_final_color = sun_color * dot(normalize(model.transform * vec4(normal, 0.0)), sun_direction);

    vec4 ambient_color = vec4(0.6, 0.6, 0.6, 1.0);

    fragColor = color * vec3(sun_final_color + sun_color * 0.4);
    // fragColor = color;
    // fragColor = vec3(model.transform * vec4(normal, 0.0));
}
