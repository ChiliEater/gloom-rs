#version 430 core

out vec4 color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;


void main()
{
    float time_dump = time; // just use `time` somewhere so that it doesn't crash
    vec4 color_1 = vec4(0.6f, 0.1f, 0.2f, 1.0f);
    vec4 color_2 = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    
    // Triangle
    vec2 vertex1 = vec2(500,400);
    vec2 vertex2 = vec2(1500,700);
    vec2 vertex3 = vec2(600,800);
    float a1 = (vertex2.y-vertex1.y)/(vertex2.x-vertex1.x);
    float a2 = (vertex3.y-vertex2.y)/(vertex3.x-vertex2.x);
    float a3 = (vertex1.y-vertex3.y)/(vertex1.x-vertex3.x);
    // does not work if there are 2 identical x values in the vertices 
    // or if the triangle is pointing downwards
    int condition = int(gl_FragCoord.y>(a1*(gl_FragCoord.x-vertex1.x)+vertex1.y))
                   *int(gl_FragCoord.y<(a2*(gl_FragCoord.x-vertex2.x)+vertex2.y))
                   *int(gl_FragCoord.y<(a3*(gl_FragCoord.x-vertex3.x)+vertex3.y));
    
    color = condition*color_1+(1-condition)*color_2;
    
}
