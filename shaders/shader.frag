#version 330 core

in vec4 color;
in vec2 TexCoords;

out vec4 final_color;

uniform sampler2D obamaTex;
uniform sampler2D lifeDuringWartime;


void main()
{
    final_color =  mix(texture(obamaTex, TexCoords),
                       texture(lifeDuringWartime, TexCoords));
}