#version 150 core

uniform vec2 displacement;
uniform vec2 scale;

in vec2 in_pos;
in vec4 in_color;

out vec4 color;

void main() {
    color = in_color;
    gl_Position = vec4(displacement + in_pos*scale, 0.0, 1.0);
}
