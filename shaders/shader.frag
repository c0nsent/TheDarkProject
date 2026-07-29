#version 330 core

in vec3 color;
in vec2 texCoords;

out vec4 final_color;

uniform sampler2D ourTexture;

void main()
{
    final_color = texture(ourTexture, texCoords);
}