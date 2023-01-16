#version 450

layout(push_constant) uniform Camera {
    vec2 tile_size;
    vec2 coordinates;
    vec2 size;
} camera;

layout(location = 0) in vec3 coordinates;
layout(location = 1) in uint texture_layer;

layout(location = 0) out uint out_texture_layer;
layout(location = 1) out vec2 uv_coordinates;

void main() {
   uint vertex_type = gl_VertexIndex % 4;
      vec2 offsets = vec2(
         vertex_type / 2,
         vertex_type % 2
      );

   vec2 instance_coordinates = vec2(
         coordinates.x - camera.coordinates.x - 1,
         coordinates.y - camera.coordinates.y
      );

   vec2 vertex_position = vec2(
      (instance_coordinates.x - instance_coordinates.y) * camera.tile_size.x / 2 + offsets.x * camera.tile_size.x,
      (instance_coordinates.x + instance_coordinates.y) * camera.tile_size.y / 4 + offsets.y * camera.tile_size.y
      );
   gl_Position = vec4(vertex_position, coordinates.z, 1.0);
   out_texture_layer = texture_layer;
   uv_coordinates = offsets;
}