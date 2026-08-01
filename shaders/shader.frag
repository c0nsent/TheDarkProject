#version 330 core

in vec4 frag_color;
in vec2 frag_tex;

out vec4 final_color;

uniform sampler2D texture0;
uniform sampler2D texture1;


void main()
{
    final_color =  mix(texture(texture0, frag_tex),
                       texture(texture1, vec2(-frag_tex.x, frag_tex.y)), 0.5);
}