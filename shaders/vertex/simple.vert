#version 430 core

in vec4 position;
layout(location = 2) in vec4 color;
out vec4 vert_color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;
uniform layout(location=3) mat4 transform;

void main()
{       float time_dump = time;
        vec4 new_position = transform * position;
        //new_position.z -= 1.0f;
        

        vert_color = color;
        gl_Position =  new_position;
}