#version 330 core

out vec4 FragColor;

in vec3 Normal_worldSpace;      // Interpolated normal for this fragment
in vec3 FragPos;     // Interpolated fragment position in world space

uniform vec3 lightPos;   // Position of the light source
uniform vec3 lightColor; // Color of the light source
uniform vec3 objectColor;    // Color of the object

void main()
{
    // Diffuse lighting calculation
    vec3 norm = normalize(Normal_worldSpace);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor * objectColor;

    FragColor = vec4(diffuse, 1.0);
}
