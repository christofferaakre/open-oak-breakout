#version 330 core

in vec2 v_tex_coords;

out vec4 frag_color;

uniform sampler2D tex;
uniform vec4 color;

void main() {
//    frag_color = vec4(1.0, 1.0, 1.0, 1.0);
    frag_color = vec4(color.xyz * texture(tex, v_tex_coords).rgb, 1.0);

}