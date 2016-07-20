#version 140
uniform sampler2D tex;
uniform float opacity;
in vec2 uv;
out vec4 color;


void main() {
    color = texture(tex, uv);
    color.a = color.a * opacity;
}
