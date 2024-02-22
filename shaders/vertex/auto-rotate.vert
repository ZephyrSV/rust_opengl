#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 texCoord;

out vec4 frontColor;
out vec2 vtexCoord;
out vec3 N;


uniform mat4 viewMatrix;
uniform mat4 modelMatrix;
uniform mat4 projectionMatrix;
uniform mat3 normalMatrix;


uniform float time;
vec4 rand_color = vec4(0.5+0.5*(gl_VertexID%2), 0.5+0.5*((gl_VertexID/2)%2), 0.5+0.5*((gl_VertexID/4)%2), 1);
                               
void main()
{
    N = normalize(normalMatrix*normal);
    frontColor = rand_color;
    vtexCoord = texCoord;
    gl_Position = projectionMatrix* viewMatrix *modelMatrix*(vec4(position, 1.0));
}
