#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoords;

out vec4 frag_color;
out vec2 frag_tex;

void main() {
    gl_Position = vec4(aPos, 1.0);
    frag_color = vec4(aColor, 1.0);
    frag_tex = aTexCoords;
}