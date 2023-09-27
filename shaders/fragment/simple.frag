#version 430 core

in vec4 vert_color;
in vec3 vert_normals;
in vec4 vert_position;

out vec4 color;

uniform layout(location=1) float time;

void main()
{
    // Light color and position (no alpha)
    vec3 light_pos = 100*vec3(cos(time/10.0),1, sin(time/10.0));
    vec3 light_color = vec3(0.9412, 0.7922, 0.7137);
    
    // Ambient lighting (base lighting for all faces)
    float ambientStrength = 0.2f;
    vec3 ambient = ambientStrength * light_color;
    
    // Diffuse lighting (lambertian model)
    vec3 light_dir = normalize(light_pos - vert_position.xyz);
    float diff = max(dot(vert_normals, light_dir), 0.0);
    vec3 diffuse = diff * light_color;

    color = vec4(vert_color.xyz * (ambient + diffuse), 1.0);
}