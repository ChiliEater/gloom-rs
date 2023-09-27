#version 430 core

in vec4 position;
in layout(location = 5) vec3 normals;
in layout(location = 2) vec4 color;

out vec4 vert_color;
out vec4 vert_position;
out vec3 vert_normals;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;
uniform layout(location=3) mat4 transform;

void main()
{       float time_dump = time;
        
        vec4 new_position = transform * position;
        
        vert_color = color;
        vert_normals = normals;
        vert_position = new_position;
        gl_Position =  new_position;
}