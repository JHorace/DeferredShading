#version 450

layout (std140) struct Light
{
    vec4 position;
    vec4 color;
//    float constant_attenuation;
//    float linear_attenuation;
//    float quadratic_attenuation;
};

layout (std140, binding = 1) uniform LightArray
{
    Light Lights[2];
};

layout(push_constant) uniform NumLights {
    uint numLights;
};


layout(location = 0) in vec3 fragPosition;
layout(location = 1) in vec3 fragNormal;
layout(location = 2) in vec3 fragColor;
layout(location = 3) in vec3 lightPosition;

layout(location = 0) out vec4 outColor;

float ambientStrength = .2f;
float specularStrength = .5f;

void main() {
    vec3 ambient = ambientStrength * vec3(Lights[1].color);

    vec3 normal = normalize(fragNormal);

    vec3 lightDirection = normalize(vec3(Lights[1].position) - fragPosition);

    float diffuse = max(dot(normal, lightDirection), 0.0f);

    vec3 diffuseColor = diffuse * vec3(Lights[1].color);

    vec3 viewDirection = normalize(vec3(Lights[1].position) - fragPosition);

    vec3 reflectDirection = reflect(-lightDirection, normal);

    vec3 specular = pow(max(dot(viewDirection, reflectDirection), 0.0), 30) * specularStrength * vec3(Lights[1].color);



    vec3 result = (ambient + diffuse + specular) * fragColor;

    outColor = vec4(result, 1.0f);

}
