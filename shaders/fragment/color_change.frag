#version 430 core
#define PI 3.141592653589793238462643383279

out vec4 color;
in vec4 vert_color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;


void main()
{
    float time_bis = mod(time/3.0,PI);
    color = (1+vec4(sin(time_bis),1-cos(2.5*time_bis),pow(sin(time_bis),2),2.0f))/2.0f;
}