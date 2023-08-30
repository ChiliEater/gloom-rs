#version 430 core

in vec3 position;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;

void main()
{
    float speed_factor = 3.0;   // the argument in the cos and sin is divided by this factor
    float time_slow = time/speed_factor;

    mat3 x_rotation = mat3(
        1,0,0,
        0,cos(time_slow),-sin(time_slow),
        0,sin(time_slow),cos(time_slow)
    );
   mat3 y_rotation = mat3(
        cos(time_slow),0,-sin(time_slow),
        0,1,0,
        sin(time_slow),0,cos(time_slow)
    );
    mat3 z_rotation = mat3(
        cos(time_slow),-sin(time_slow),0,
        sin(time_slow),cos(time_slow),0,
        0,0,1
    );

    mat3 rotation = x_rotation * y_rotation * z_rotation;     // rotation applied to the vertices


    gl_Position = vec4(rotation*position, 1.0f);
}