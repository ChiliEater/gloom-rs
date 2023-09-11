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