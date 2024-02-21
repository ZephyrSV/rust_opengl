#version 330
out vec4 FragColor;
in vec3 vertexColor;
uniform float time;

void main() {
    FragColor = vec4(0.5+0.5*sin(time), 1.0, 1.0, 1.0);
}
