#version 430 core

out vec4 color;
in vec4 vert_color;
in noperspective vec4 vert_position;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;

/* uses screen coordinates to draw checkerboard, does not follow perspective
void main()
{
    float time_dump = time; // just use `time`somewhere so that it doesn't crash
    vec4 color_1 = vec4(0.6f, 0.1f, 0.2f, 1.0f);
    vec4 color_2 = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    
    // Checkerboard
    int size = 40;
    int condition = int(mod(floor(gl_FragCoord.x/size),2)==mod(floor(gl_FragCoord.y/size),2));
    
    color = condition*color_1+(1-condition)*color_2;
    
}*/


// Uses vertices coordinates to draw the checkerboard, follows perspective
void main()
{
    //vec4 color;
    float time_dump = time; // just use `time` somewhere so that it doesn't crash
    vec4 color_1 = vec4(0.6f, 0.1f, 0.2f, 1.0f);
    vec4 color_2 = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    
    // Checkerboard
    int size = 5;
    bool pattern = (mod(floor(vert_position.x*size),2)
    ==mod(floor(vert_position.y*size),2));
    
    color = (pattern) ? color_1 : color_2;
    
}