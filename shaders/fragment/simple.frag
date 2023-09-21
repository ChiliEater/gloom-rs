#version 430 core

in noperspective vec4 vert_color;
out vec4 color;

void main()
{
    color = vert_color;
}
