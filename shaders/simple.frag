#version 430 core

out vec4 color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;


void main()
{
    float time_dump = time; // just use `time`somewhere so that it doesn't crash
    color = vec4(0.6f, 0.1f, 0.2f, 1.0f);
}
