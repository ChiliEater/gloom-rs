#version 430 core

in vec4 vert_color;
in vec3 vert_normals;
in vec4 vert_position;
in vec4 vert_new_position;

out vec4 color;

uniform vec4 camera_position;
uniform float elapsed_time;
uniform vec4 heli_position;
uniform mat4 heli_yaw;

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
    // ambient lighting
    vec3 ambient_color = vec3(0.9216, 0.4431, 0.1451);
    float ambient_coeff = 0.2;
    vec3 ambient = ambient_coeff * ambient_color;
        
    // Lambertian lighting (light in front of the helicopter.)
    vec4 light_pos = heli_position + heli_yaw * vec4(0.0,10.0,20.0,1.0);
    //vec4 light_pos = vec4(100,100,100,1.0);
    vec3 light_color = vec3(0.0588, 0.5608, 0.6804) * 0.9;
    vec3 light_dir = normalize((light_pos - vert_new_position)).xyz;
    float diff_coeff = max(dot(vert_normals, light_dir), 0.0);
    vec3 diffuse = diff_coeff * light_color;

    // Specular lighting (helicopter light)
    vec3 view_dir = normalize(camera_position.xyz-vert_new_position.xyz);
    vec3 halfway_dir = normalize(light_dir + view_dir);
    int shininess = 1000;
    float spec = pow(max(dot(vert_normals, halfway_dir), 0.0), shininess);
    vec3 specular_color = vec3(1.0, 1.0, 1.0);
    vec3 specular = spec * specular_color;

    // Lambertian lighting (rotating sun)
    vec4 sun_pos = 1000 * vec4(3*cos(elapsed_time),sin(elapsed_time),3,1.0);
    vec3 sun_color = ambient_color;
    vec3 sun_dir = normalize((sun_pos - vert_new_position)).xyz;
    float sun_diff_coeff = max(dot(vert_normals, sun_dir), 0.0);
    vec3 sun_diffuse = sun_diff_coeff * sun_color;

    // Specular lighting (sun light)
    vec3 sun_halfway_dir = normalize(sun_dir + view_dir);
    int sun_shininess = 100;
    float sun_spec = pow(max(dot(vert_normals, sun_halfway_dir), 0.0), sun_shininess);
    vec3 sun_specular_color = vec3(1.0, 1.0, 1.0);
    vec3 sun_specular = sun_spec * sun_specular_color;


    // Implement fog (flat color)
    float dist =  distance(camera_position,vert_new_position);
    vec3 fog_color = ambient_color;
    float fog_density = getFogFactor(dist); 
    vec3 fog = fog_density*fog_color;

    color = vec4(vert_color.xyz * ((1-fog_density)*(diffuse+ambient+specular+sun_diffuse+sun_specular)+fog), 1.0);

}