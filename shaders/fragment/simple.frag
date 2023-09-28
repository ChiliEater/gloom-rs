#version 430 core

in vec4 vert_color;
in vec3 vert_normals;
in vec4 vert_position;

out vec4 color;

uniform layout(location=1) float time;

void main()
{
    // Fixed light source, not really ambient lighting but adds volume.
    vec3 ambient_dir = normalize(vec3(0.8,0.5,0.6));
    vec3 ambient_color = vec3(0.0078, 0.302, 0.251);
    float ambient_coeff = max(dot(vert_normals, ambient_dir), 0.0);
    vec3 ambient = ambient_coeff * ambient_color;

    // Ambient lighting (base lighting for all faces)
    //float ambientStrength = 0.2f;
    //vec3 ambient = ambientStrength * ambient_color;
    
    // Diffuse lighting (lambertian model)
    
    // Light color and position (rotating Sun)
    vec3 light_pos = 1000*vec3(2*cos(time/10.0), sin(time/10.0),-1);
    vec3 light_color = vec3(0.9216, 0.4431, 0.1451);
    
    vec3 light_dir = normalize(light_pos - vert_position.xyz);
    float diff_coeff = max(dot(vert_normals, light_dir), 0.0);
    vec3 diffuse = diff_coeff * light_color;

    color = vec4(vert_color.xyz * (ambient + diffuse), 1.0);
}