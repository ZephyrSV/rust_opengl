#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec3 color;
layout (location = 3) in vec2 texCoord;

out vec4 frontColor;
out vec2 vtexCoord;

uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;
uniform mat3 normalMatrix;

uniform float time;
uniform float speed= .5;
uniform float PI = 3.14;
float a = time*speed;
mat4 autoRotate = mat4( cos(a) , 0, -sin(a), 0,
                        0      , 1, 0     , 0,
                        sin(a), 0, cos(a), 0,
                        0      , 0, 0     , 1);
                               
void main()
{
    vec3 N = normalize(normalMatrix * normal);
    frontColor = vec4(color,1.0);
    vtexCoord = texCoord;
    gl_Position = projectionMatrix* viewMatrix *modelMatrix*(autoRotate * vec4(vertex, 1.0));
}
