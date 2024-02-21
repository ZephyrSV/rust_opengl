#version 330 core

layout(location = 0) in vec3 position;
uniform float time;

void main()
{
    gl_Position = vec4(position, 1.0) + vec4(sin(time), 0,0,0);
}
