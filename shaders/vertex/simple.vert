#version 430 core

in vec4 position;
uniform layout(location=1) float time;
layout(location = 2) in vec4 color;
out vec4 vert_color;

void main()
{
    vert_color = color;
    gl_Position = position;
}