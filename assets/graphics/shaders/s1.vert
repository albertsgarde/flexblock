#version 330 core

layout(location=0) in vec3 vertexPosition_modelspace;
layout(location=1) in vec3 incolor;
layout(location=2) in vec2 inUV;
out vec3 vertexcolor;
out vec2 UV;


uniform mat4 MVP;

void main() {
	gl_Position = MVP * vec4(vertexPosition_modelspace, 1);
	float dist = clamp(abs(gl_Position.w/50),0,1);

	vec3 fogcolor = vec3(0.6, 0.6, 0.6);

	vertexcolor = incolor * (1-dist) + fogcolor*dist;
	UV = inUV;
}