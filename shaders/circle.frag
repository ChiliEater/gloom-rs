#version 430 core

out vec4 color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;


void main()
{
    float time_dump = time; // just use `time`somewhere so that it doesn't crash
    vec4 color_1 = vec4(0.6f, 0.1f, 0.2f, 1.0f);
    vec4 color_2 = vec4(1.0f, 1.0f, 1.0f, 1.0f);

    // Circle
    int amplitude = 100*100;
    int size = 200;
    int centerX=800;
    int centerY=600;
    int condition = int((gl_FragCoord.x-centerX)*(gl_FragCoord.x-centerX)
    + (gl_FragCoord.y-centerY)*(gl_FragCoord.y-centerY) < size*size);

    color = condition*color_1+(1-condition)*color_2;
    
}
