#version 330 core

in vec3 vertexcolor;
in vec2 UV;

uniform sampler2D tex;

out vec4 color;


void main() {
    color = texture(tex, UV);
    if(color.a < 0.3)
        discard;
    color = vec4(color.xyz * vertexcolor, color.a);
}