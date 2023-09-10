#version 430 core

in vec4 position;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;

void main()
{
    float d = -1.0;           // Distance for the perspective projection
    
    float speed_factor = 3.0;   // the argument in the cos and sin is divided by this factor
    float time_slow = time/speed_factor;

    // Perspective projection matrix
    mat4 perspective = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 1.0/d, 0.0
    );
    
    vec4 new_position = position;
    new_position.z -= 1.0f;

    vec4 new_projection = vec4(new_position*perspective);

    gl_Position =  vec4(new_projection/new_projection.w);
}