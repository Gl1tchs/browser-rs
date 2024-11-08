#version 450 core

in vec2 position;
in vec4 color;

out vec4 v_color;

uniform mat4 view;
uniform mat4 proj;

void main() {
    v_color = color;

    gl_Position = proj * view * vec4(position, 0.0, 1.0);
}
