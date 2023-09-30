#version 430 core

in vec4 position;
in layout(location = 5) vec3 normals;
in layout(location = 2) vec4 color;

out vec4 vert_color;
out vec4 vert_position;
out  vec3 vert_normals;
out mat4 transform_mat;
out vec4 orig_vert;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;
uniform mat4 transform;

void main()
{       float time_dump = time;
        
        vec4 new_position = transform * position;
        
        vert_color = color;
        vert_normals = normals;
        vert_position = new_position;
        orig_vert = position;
        gl_Position =  new_position;
        transform_mat = transform;
}