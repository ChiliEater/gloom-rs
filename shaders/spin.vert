#version 430 core

in vec4 position;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;

void main()
{
    float speed_factor = 3.0;   // the argument in the cos and sin is divided by this factor
    float time_slow = time/speed_factor;

    mat4 x_rotation = mat4(
        1,0,0,0,
        0,cos(time_slow),-sin(time_slow),0,
        0,sin(time_slow),cos(time_slow),0,
        0,0,0,1
    );
    mat4 y_rotation = mat4(
        cos(time_slow),0,-sin(time_slow),0,
        0,1,0,0,
        sin(time_slow),0,cos(time_slow),0,
        0,0,0,1
    );
    
    mat4 rotation = x_rotation * y_rotation;     // rotation applied to the vertices

    float d = -1.0;           // Distance for the perspective projection

    // Perspective projection matrix
    mat4 perspective = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 1.0/d, 0.0
    );
    
    vec4 new_position = rotation * position;
    new_position.z -= 1.0f;

    vec4 new_projection = vec4(new_position*perspective);

    gl_Position =  vec4(new_projection/new_projection.w);
}