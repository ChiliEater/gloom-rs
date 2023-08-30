#version 430 core

out vec4 color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;


void main()
{
    vec4 color_1 = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    vec4 color_2 = vec4(0.972f, 0.992f, 0.007f, 1.0f);

    // Color change
    float condition = (1.0+sin(time)) / 2.0f;
    color = condition*color_1+(1-condition)*color_2;
    
}
