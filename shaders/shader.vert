#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

uniform float offset;

out vec3 color;
out vec3 pos;

void main() {
    gl_Position = vec4(aPos.x + offset, aPos.y, aPos.z, 1.0);

    pos = aPos;

    color = aColor + offset;
}