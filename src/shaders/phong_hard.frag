#version 450

layout(location = 0) in vec3 fragPosition;
layout(location = 1) in vec3 fragNormal;
layout(location = 2) in vec3 fragColor;
layout(location = 3) in vec3 lightPosition;

layout(location = 0) out vec4 outColor;

vec3 lightColor = vec3(1.0, 1.0, 1.0);

float ambientStrength = .2f;
float specularStrength = .5f;

void main() {
    vec3 ambient = ambientStrength * lightColor;

    vec3 normal = normalize(fragNormal);

    vec3 lightDirection = normalize(lightPosition - fragPosition);

    float diffuse = max(dot(normal, lightDirection), 0.0f);

    vec3 diffuseColor = diffuse * lightColor;

    vec3 viewDirection = normalize(lightPosition - fragPosition);

    vec3 reflectDirection = reflect(-lightDirection, normal);

    vec3 specular = pow(max(dot(viewDirection, reflectDirection), 0.0), 30) * specularStrength * lightColor;



    vec3 result = (ambient + diffuse + specular) * fragColor;

    outColor = vec4(result, 1.0f);
}
