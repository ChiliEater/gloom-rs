#version 430 core

in vec4 position;
uniform layout(location=1) float time;
layout(location = 2) in vec4 color;
out vec4 vert_color;

void main()
{
    float time_dump = time;
    vert_color = color;
    float scale = 0.5;
    vec4 new_position = vec4(position.x*scale,position.y*scale,position.z*scale,position.w);
    gl_Position = new_position;
}