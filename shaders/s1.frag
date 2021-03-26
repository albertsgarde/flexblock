#version 330 core

in vec3 vertexcolor;
in vec2 UV;

uniform sampler2D testTexture;

out vec3 color;


void main() {
    color = vertexcolor;
}