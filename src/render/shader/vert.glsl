#version 460

layout(location = 0) in vec2 position;

uniform mat3 transform;
uniform mat4 projection;

void main() { gl_Position = projection * vec4(transform * vec3(position, 1.0), 1.0); }