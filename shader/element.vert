#version 150 core

in vec2 in_pos;
in vec4 in_color;

out vec4 color;

void main() {
    color = in_color;
    gl_Position = vec4(in_pos, 0.0, 1.0);
}
