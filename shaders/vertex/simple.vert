#version 430 core

in vec4 position;
in layout(location = 5) vec3 normals;
in layout(location = 2) vec4 color;

out vec4 vert_color;
out vec4 vert_position;
out vec4 vert_new_position;
out  vec3 vert_normals;
out mat4 transform_mat;


// Uniform variables that need to be updated in the rendering loop
uniform float elapsed_time;
uniform mat4 transform;
uniform mat4 view_projection;
uniform vec4 camera_position;


void main()
{       
        vec4 new_position = view_projection * transform * position;
        
        vert_color = color;
        vert_normals = normalize(mat3(transform) * normals);
        vert_position = position;
        vert_new_position = transform * position;
        gl_Position =  new_position;
        
}