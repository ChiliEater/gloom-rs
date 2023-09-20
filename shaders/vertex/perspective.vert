#version 430 core

in vec4 position;
layout(location = 2) in vec4 color;
out vec4 vert_color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;

void main()
{       float time_dump = time;
        float d = -0.8;           // Distance for the perspective projection  

        // Perspective projection matrix
        mat4 perspective = mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 1.0/d,
            0.0, 0.0, 0.0, 0.0
        );
         
        float n = -1.0;
        float f = -2.0;
        
        mat4 Pvt = mat4(
            n, 0.0, 0.0, 0.0,
            0.0, n, 0.0, 0.0,
            0.0, 0.0, n+f, 1.0,
            0.0, 0.0, -n*f, 0.0
        );

        vec4 new_position = position;
        new_position.z -= 1.0f;
        

        vec4 new_projection = Pvt * perspective * new_position;
        vert_color = color;
        gl_Position =  vec4(new_projection/new_projection.w);
        
}