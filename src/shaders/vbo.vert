#version 450

layout(location = 0) in vec4 vertPosition;
layout(location = 1) in vec4 vertColor;
layout(location = 2) in vec2 vertUV;

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = vertPosition;
    fragColor = vec3(1.0, 0.0, 0.0);
}
