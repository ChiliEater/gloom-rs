#version 430 core

in vec4 position;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;

void main()
{
    float d = 5.0;            // Distance for the perspective projection
    
    float speed_factor = 3.0;   // the argument in the cos and sin is divided by this factor
    float time_slow = time/speed_factor;

    // Rotations not used yet
    /*mat4 x_rotation = mat4(
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
    );*/

    // Perspective projection matrix
    mat4 perspective = mat4{
        1,0,0,0,
        0,1,0,0,
        0,0,1,0,
        0,0,1/d,0
    }

    vec4 new_projection = vec4(perspective*position)

    gl_Position =  vec4(new_projection/new_projection.w);     
}