#version 330 core

in vec3 vertexcolor;
in vec2 UV;

uniform sampler2D test_texture;

out vec3 color;


void main() {
    color = vertexcolor * texture(test_texture, UV).xyz;
}