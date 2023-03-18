#version 450

layout(location = 0) in vec2 pos;
layout(location = 1) in vec3 color;

layout(location = 0) out vec3 fragColor;

void main()
{
    int x_coord = gl_InstanceIndex % 20;
    int y_coord = gl_InstanceIndex / 20;
    vec2 current_instance_coords = vec2(x_coord, y_coord);
    vec2 newPos = vec2(gl_InstanceIndex % 60 / 30. + pos.x, gl_InstanceIndex / 60 / 30. + pos.y);
 
    gl_Position = vec4(newPos, 0.0, 1.0);
    fragColor = color;
}