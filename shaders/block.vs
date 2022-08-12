#version 330 core

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform mat4 model;

void main() {
    gl_Position = model * vec4(position, 1.0, 1.0);
    v_tex_coords = tex_coords;

}
