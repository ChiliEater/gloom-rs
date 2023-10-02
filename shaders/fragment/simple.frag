#version 430 core

in vec4 vert_color;
in vec3 vert_normals;
in vec4 vert_position;
in vec4 vert_new_position;

out vec4 color;

uniform layout(location=1) float time;
uniform vec4 camera_position;

float getFogFactor(float d)
{
    const float fog_max_distance = 1000.0;
    const float fog_min_distance = 200.0;

    if (d>=fog_max_distance) return 1;
    if (d<=fog_min_distance) return 0;

    return 1-(fog_max_distance - d) / (fog_max_distance - fog_min_distance);
}



void main()
{
    // Diffuse lighting (lambertian model)
    
    // Fixed light source at infinity, not really ambient lighting but adds volume.
    vec3 ambient_dir = normalize(vec3(0.8,0.5,0.6));
    vec3 ambient_color = vec3(0.9216, 0.4431, 0.1451);
    float ambient_coeff = max(dot(vert_normals, ambient_dir), 0.0);
    vec3 ambient = ambient_coeff * ambient_color;
        
    // Light color and position (rotating Sun)
    vec4 light_pos =   500 * vec4(2*cos(time/10.0), sin(time/10.0),-1,1.0);
    //vec4 light_pos = camera_position;
    vec3 light_color = vec3(0.0588, 0.5608, 0.7804);
    //vec3 light_color = vec3(0.9, 0.9, 0.6);

    vec3 light_dir = normalize((light_pos - vert_new_position)).xyz;
    float diff_coeff = max(dot(vert_normals, light_dir), 0.0);
    vec3 diffuse = diff_coeff * light_color;

    // Specular lighting (Phong's model)
    vec3 view_dir = normalize(camera_position.xyz-vert_new_position.xyz);

    vec3 halfway_dir = normalize(light_dir + view_dir);

    int shininess = 100;
    float spec = pow(max(dot(vert_normals, halfway_dir), 0.0), shininess);
    vec3 specular = light_color * spec;

    // Implement fog (flat color)
    float dist =  distance(camera_position,vert_new_position);
    vec3 fog_color = ambient_color;
    float fog_density = getFogFactor(dist); 
    vec3 fog = fog_density*fog_color;

    color = vec4(vert_color.xyz * ((1-fog_density)*(diffuse+ambient+specular)+fog), 1.0);
}