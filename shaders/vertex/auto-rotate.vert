#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout (location = 2) in vec2 texCoord;

out vec3 FragPos;     // FragPos is the position of the fragment in world space
out vec3 Normal_worldSpace;      // Normal in world space

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * model * vec4(position, 1.0);
    FragPos = vec3(model * vec4(position, 1.0));
    Normal_worldSpace = mat3(transpose(inverse(model))) * normal; // Transform normal to world space
}