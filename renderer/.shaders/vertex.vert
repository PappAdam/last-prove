#version 450

#define PI 3.141592653589793238462643383279502884197169

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(binding = 0) uniform rotation {
    vec3 rotation;
} transform;

layout(location = 0) out vec3 fragColor;

void main()
{
    float alpha = transform.rotation.z;

    mat2 rot = mat2(
        sin(alpha), sin(PI / 2 + alpha),
        cos(alpha), cos(PI / 2 + alpha)
    );

    gl_Position = vec4(rot * pos.xy, pos.z, 1.0);
    fragColor = color;
}