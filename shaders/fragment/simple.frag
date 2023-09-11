#version 430 core

in vec4 vert_color;
in vec3 vert_normal;
out vec4 color;


void main()
{
    color = vec4(vert_color.xyz * vert_normal, vert_color.w);
}
