#version 450

layout(location = 0) in vec3 vertPosition;
layout(location = 1) in vec3 vertNormal;
layout(location = 2) in vec3 vertColor;
layout(location = 3) in vec2 vertUV;

layout(binding = 0) uniform UBO {
    mat4 model;
    mat4 view;
    mat4 projection;
} ubo;

layout(location = 0) out vec3 fragPosition;
layout(location = 1) out vec3 fragNormal;
layout(location = 2) out vec3 fragColor;
layout(location = 3) out vec3 lightPosition;

void main() {
    fragPosition = vec3(ubo.model * vec4(vertPosition, 1.0));

    fragNormal = mat3(transpose(inverse(ubo.model))) * vertNormal;

    fragColor = vec3(1.0, 0.0, 0.0);

    lightPosition = vec3(10, 10, 10);

    gl_Position = ubo.projection * ubo.view * ubo.model * vec4(vertPosition, 1.0);

}
