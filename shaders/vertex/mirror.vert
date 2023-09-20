#version 430 core

in vec4 position;
layout(location = 2) in vec4 color;
out vec4 vert_color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;

void main()
{
    float time_dump = time;
    vert_color = color;
    gl_Position = vec4(position.x * -1.0f, position.y * -1.0f, position.z, position.w);
}