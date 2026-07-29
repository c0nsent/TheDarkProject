#version 330 core

in vec4 color;
in vec2 TexCoords;

out vec4 final_color;

uniform sampler2D ourTexture;

void main()
{
    final_color = texture(ourTexture, TexCoords);
}