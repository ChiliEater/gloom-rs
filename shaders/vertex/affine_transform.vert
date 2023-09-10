#version 430 core


in vec4 position;
uniform layout(location=1) float time;
layout(location = 2) in vec4 color;
out vec4 vert_color;

void main()
{   float time_dump = time;


    float a = 1;
    float b = 1*0;
    float c = 1*0;
    float d = 1*0;
    float e = 1;
    float f = 1*0;

    mat4 A = mat4(
        a, d, 0, 0,
        b, e, 0, 0,
        0, 0, 1, 0,
        c, f, 0, 1
    );

    vert_color = color;
    gl_Position = A*position;
}