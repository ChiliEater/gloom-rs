---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Exercise 2
author:
- Jonas Joshua Costa
- Noé Hirschauer
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=2cm
toc: false
toc-title: "Table of Contents"
toc-depth: 2
numbersections: true
header-includes:
# The `atkinson` font, requires 'texlive-fontsextra' on arch or the 'atkinson' CTAN package
# Uncomment this line to enable:
- '`\usepackage[sfdefault]{atkinson}`{=latex}'
colorlinks: true
links-as-notes: true
# The document is following this break is written using "Markdown" syntax
---

# Task 1

In this task we will focus on loading the terrain model and implement some simple lighting. We use the given `mesh.rs` file to extract the mesh from the `lunarsurface.obj` file.

We also modified our VAO generation function to add a new buffer of floats containing normals.

## Task 1c

Now that our VAO supports normals, we can get them in the vertex shader and pass them to the fragment shader to do implement lighting.

In the vertex shader:

```glsl
in layout(location = 5) vec3 normals;
out vec3 vert_normals;
...
vert_normals = normals;
```

First, we will visualize normals with the fragment shader by using the $x,y,z$ coordinates of the normals as the RGB component of the vertices :

```glsl
in vec3 vert_normals;
out vec4 color;
...
color = vec4(vert_normals.xyz,1.0);
```

Since most of the triangles are facing "up" (positive $y$) so the main color will be green as shown in the picture below.

![First look at a crater with the colors set to the normals components](img/crater_RGB_normals.png)

## Task 1d

The image above is not exactly what one would expect the surface of the moon to look like. To finally get some correct shading, we will implement a simple lambertian lighting model. 

The basic idea is that we have a light source at a given position and we compute the vector corresponding to the direction of this light source for each fragment. We obtain a vector $v_\mathrm{light\,direction}$. The color of the fragment is then set to :

$$\mathrm{newcolor} = \mathrm{color_{RGB}} \times max(0,-v_\mathrm{normal}\cdot v_\mathrm{light\,direction})$$

In our shader we implemented a slightly more complex model with ambient lighting (base level of lighting) and a light color. Also, the light source is spinning above the camera so

```glsl
void main()
{
    // Light color and position (no alpha)
    vec3 light_pos = 100*vec3(cos(time/10.0),1, sin(time/10.0));
    vec3 light_color = vec3(0.9412, 0.7922, 0.7137);
    
    // Ambient lighting (base lighting for all faces)
    float ambientStrength = 0.2f;
    vec3 ambient = ambientStrength * light_color;
    
    // Diffuse lighting (lambertian model)
    vec3 light_dir = normalize(light_pos - vert_position.xyz);
    float diff = max(dot(vert_normals, light_dir), 0.0);
    vec3 diffuse = diff * light_color;

    color = vec4(vert_color.xyz * (ambient + diffuse), 1.0);
}
```

We end up with the following result:

![Smooth shading on the moon.](img/moon_shading.png)

# Task 2c
We constructed a scene graph that contains all the elements of the scene. The structure is shown in the figure below:

![](img/scene_graph.png)

**mermaid code (might not work with pandoc)**

```mermaid
graph TD;
    R((ROOT)) --> TERRAIN;
    R --> H[HELICOPTER];
    H --> M(main rotor);
    H --> T(tail rotor);
    H --> D(door);
```

The `SceneNode` structure has the following properties:

- `position` where the node should be in relation to their parent
- `rotation` of the node relative their parent
- `scale` is quite explicit
- `reference_point` the point this node is supposed to scale and rotate around
- `vao_id` contains the vao of what should be drawn in this node
- `index_count` the number of vertices that should be drawn
- `children` contains all the scene node "below"

We can now recursively iterate over all nodes to get their transformations relative to their parents and draw the elements in the scene using the following function.

```rust
    unsafe fn draw_scene(
        &mut self,
        node: &SceneNode,
        view_projection_matrix: &Mat4x4,
        transformation_so_far: &Mat4x4,
    ) {
        // Determine the transformation matrix from the current state
        let new_matrix: Mat4x4 = transformation_so_far * node.get_transform_intrinsic();
        // Only draw if we have faces
        if node.index_count > 0 && node.node_type == NodeType::Mesh {
            gl::BindVertexArray(node.vao_id);
            match &self.shader {
                Some(shader) => {
                    // Pass matrices to our shaders
                    let transform_uniform = shader.get_uniform_location("transform");
                    gl::UniformMatrix4fv(transform_uniform, 1, gl::FALSE, new_matrix.as_ptr());
                    let view_uniform = shader.get_uniform_location("view_projection");
                    gl::UniformMatrix4fv(
                        view_uniform,
                        1,
                        gl::FALSE,
                        view_projection_matrix.as_ptr(),
                    );
                }
                None => {}
            }
            // Draw
            gl::DrawElements(
                gl::TRIANGLES,
                node.index_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
        // Recursive call
        for &child in &node.children {
            self.draw_scene(&*child, view_projection_matrix, &new_matrix);
        }
    }
```

We get the following scene

![](img/helicopter_scene.png)

# Task 5

## Task 5a

There is something wrong about the lighting when the helicopter rotates around as shown in the images below

![Helicopter without rotation has correct lighting](img/heli-good.png)

![If the helicopter is rotated, the lighting is wrong.](img/heli-wrong.png)

## Task 5c

We split the transform matrix into 2 parts that are passed as uniforms to the vertex shader:

- View projection matrix that transforms the vertex positions.
- Model matrix that is used to transform the vertex normals.

```glsl
in vec4 position;               // The vertex positions
uniform mat4 transform;         // Normals transformation matrix
uniform mat4 view_projection;   // View projection matrix

out vert_normals                // The updated normals
void main(){
        vert_normals = normalize(mat3(transform) * normals);
        gl_Position =  view_projection * transform * position;  
}
```
This allows us to use the corrected `vert_normals` in the fragment shader to fix the lighting.

![](img/heli-lighting-1-final.png)

![](img/heli-lighting-2.png)

# Task 6

![Here we can see 5 helicopters in the wild.](img/5-helis.png)

![The above screenshot shows a hover of helicopters containing 100 individuals!](img/100-helis.png)

# Bonus Tasks

## Bonus a

## Bonus b/c

Making the helicopter controllable was suprisingly simple. We just have to apply the movement that's calculated for the camera to the helicopter instead. The scene graph takes care of the rest.

Making it nice was *significantly* more difficult, though. Instead of just adding a fixed value, we replaced the movement system with rudimentary momentum based one. Pressing keys simply accelerates the speed vector which is then passed into another function, along with some other info, to produce the helicopter's next position. We also made the camera rotate around the helicopter. When no keys are pressed, the helicopter decelerates until it reaches a standstill.

## Bonus d

We didn't do this one.

## Bonus e

To make the helicopter more visually appealing, we changed the animation function to also output pitch, yaw, roll. This is where we finally ran into issues with extrinsic angles. This lead us to ~~a lot of frustration~~ refactor a few parts of the code. In the end, we ended up with a setup that works with camera, movement, and animations.

## Bonus f

![The pilot is a lie?](img/easter-egg.png)

## Bonus g

We implemented really simple fog in the fragment shader. We also added some curvature to the surface to make it look at least a little like a small planet. There's no gravity though!

![](img/the-fog.png)