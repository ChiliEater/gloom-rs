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
        // Apply radial distortion to create a spherical world effect
        float planet_radius = 1500;
        // Apply the spherical deformation
        vec3 sphere_center = vec3(0.0, -planet_radius, 0.0); 
        vec3 surface_normal = normalize((transform*position).xyz - sphere_center);

        // Calculate the displacement based on the distance from the center of the sphere
        vec3 position_on_sphere = vec3(position.x,0,position.z);
        float distance_to_center = length(position_on_sphere - sphere_center);
        vec3 deform_offset = (planet_radius - distance_to_center)*surface_normal;

        // Apply the deformation to the position
        vec4 deformed_position = position + vec4(deform_offset, 0.0);

        // Transform the deformed position as before
        vec4 new_position = view_projection * transform * deformed_position;

        vert_color = color;
        vert_normals = normalize(mat3(transform) * normals + surface_normal);
        vert_position = deformed_position;
        vert_new_position = transform * deformed_position;
        gl_Position = new_position;
}