#version 330 core

layout(location=0) in vec3 vertexPosition_modelspace;
layout(location=1) in vec3 incolor;
layout(location=2) in vec2 inUV;
out vec3 vertexcolor;



void main() {
	gl_Position = vec4(vertexPosition_modelspace, 1);
	vertexcolor = incolor;
}