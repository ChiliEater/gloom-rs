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
uniform layout(location=1) float time;
uniform mat4 transform;
uniform mat4 view_projection;


void main()
{       float time_dump = time;
        // This is the normal shader
        vec4 new_position = view_projection * transform * position;
        
        vert_color = color;
        vert_normals = normalize(mat3(transform) * normals);
        vert_position = position;
        vert_new_position = transform * position;
        gl_Position =  new_position;
        
        
        /*
        // Apply radial distortion to create a spherical world effect
        float sphere_radius = 1000;
        vec3 sphere_center = (transform * vec4(0,-1000,0,1.0)).xyz;
        vec3 original_position = position.xyz - sphere_center;
        float distance = length(original_position);
        vec3 normalized_position = original_position / distance;
        vec3 distorted_position = normalized_position * sphere_radius;
        
        // Transform the distorted position
        vec4 new_position = view_projection * transform * vec4(distorted_position, 1.0);
        
        vert_color = color;
        vert_normals = normalize(mat3(transform) * normals);
        vert_position = position;
        vert_new_position = transform * vec4(distorted_position,1.0);
        gl_Position = new_position;*/
}