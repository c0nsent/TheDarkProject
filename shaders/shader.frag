#version 330 core

uniform vec4 uni_color;

in vec3 color;
in vec3 pos;


out vec4 final_color;



void main()
{
    final_color = vec4(pos, 1.0);
}