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
geometry: margin=4cm
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


Note: Use Q and E to change fragment shaders, A and D to change models, Y and C to change vertex shaders.

# Task 1b

![](img/rgb-cube.png)

OpenGL makes use of barycentric interpolation which, in simple terms, simply computes a weighted sum of all three color values with the distances from the fragement to the vertices being the weight. [SO](https://stackoverflow.com/questions/13210998/opengl-colour-interpolation13211355) [Wikipedia](https://en.wikipedia.org/wiki/Barycentric_coordinate_system#Barycentric_coordinates_on_triangles)

# Task 2
In this task we will focus on 3 overlapping triangles with different colors and a given transparency. The triangles are in different $z$ planes. 

**a)**

First, we make sure that the triangles are drawn back to front. In our case, the order is **red** $\rightarrow$ **green** $\rightarrow$ **blue** as shown in the picture below :

![](img/triangles_back_to_front.png)

The colors of the triangles are :
- pure red `(1,0,0)`, 33% transparency
- pure green `(0,1,0)`, 33% transparency
- pure blue `(0,0,1)`, 33% transparency
  
The part where the triangles overlap is mostly blue, which is the color of the closest triangle (last one being drawn).

**b1)**

Now we will swap the color of the triangles. The $z$ position and the drawing order of the triangles remains the same, only the color changes.
  
- **red** $\rightarrow$ **blue** $\rightarrow$ **green**
  ![](img/triangles_RBG.png)
  In this case, the overlapping area is mostly green
- **green** $\rightarrow$ **blue** $\rightarrow$ **red**
  ![](img/triangles_GBR.png)
  In this case, the overlapping area is mostly red

These are expected resultes considering how OpenGL computes the color with alpha blending :

$$\mathrm{Color_{new}}=\alpha_\mathrm{source}\times \mathrm{Color_{source}} + (1-\alpha_\mathrm{source})\times \mathrm{Color_{destination}}$$

Here, we have the source alpha set to 33% so, on a pure white background, the color of a triangle will be white + some color which is why the triangle look very light.

In the places where they overlap, the destination color is the mix of white and color mentionned above, which will be added to the new source color. For example, in the area where red and blue overlap in the last image, there is a pure blue with 33% transparency added to a light green area which results in a cyan-blue color.

**b2)**

Now, we will change the $z$ coordinate of the triangles without changing the color. The reference order is the one in **Task 2a** (back to front : red $\rightarrow$ green $\rightarrow$ blue)

- **back** $\rightarrow$ **front** $\rightarrow$ **middle**
  
- **middle** $\rightarrow$ **front** $\rightarrow$ **back**
## Issue : 2 vertices of blue triangle are completely hidden so it does not render. Need to tweak it a little i think

# Task 3
For the rest of **Task 3** the transformations will be compared to the following reference image :

![](img/a1.png)

#### a)
It is possible to multiply each vertex by a matrix using the vertex shader :

```glsl
in vec4 position;

void main()
{
    mat4 A = mat4(
        1, 1, 0, 0,
        0, 0.5, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    );
    gl_Position = A*position;
}
```
The output vertices give the following image :

![](img/3a-monkey.png)

#### b)

In this section we will study the impact of modifying one of $a,b,c,d,e,f$ in the following affine transformation matrix :
$$
A = \begin{bmatrix}
a&b&0&c\\
d&e&0&f\\
0&0&1&0\\
0&0&0&1
\end{bmatrix}
$$

To better see the effect of changing only one variable (starting from the identity matrix), we will use a uniform variable oscillating between -1 and 1.

- $a$ and $e$ impacts the scaling of the $x$ anf $y$ coordinate respectively as shown below. It should be  noted that having a negative value flips the projection plane.
  
![](img/a3.png)
*$a$ : x scaling*

![](img/var-e.png)

*$e$ : y scaling*

- $b$ and $d$ correspond to shear along the $x$ and $y$ axis respectively 

![](img/b.png)

*$b$ : shearing along x*


![](img/var-d.png)

*$d$ : shearing along y*

- $c$ and $f$ correspond to translations along the $x$ and $y$ axis respectively. It is the expected behavior of the trasform vector in the homogenous coordinates.

![](img/var-c.png)

*$c$ : horizontal translation*

![](img/var-f.png)

*$f$ : vertical translation*

#### c)
To define a rotation of the whole model, we need a rotation matrix such as $R_x$ for a rotation around the $x$ axis:

$$
R_x = \begin{bmatrix}
\cos\theta&-\sin\theta&0&0\\
\sin\theta&\cos\theta&0&0\\
0&0&1&0\\
0&0&0&1
\end{bmatrix}
$$

In a rotation matrix, there are 4 coordinates that need to be changed at the same time, unlike in **Task 3b)** where only one value was changed. 