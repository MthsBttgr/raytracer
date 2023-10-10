# Raytracer
This project is me learning about how raytracers work, and trying to build one myself. 
My raytracer is insanly basic, but pretty cool in my opinion aswell. 
It simulates a virtual camera, from which a bunch of rays are sent out into a virtual world.
The rays interact and bounce around within this world, and the data is used to construct an image.
The image format is .ppm, which isn't great, but very easy to work with.

I followed the guide: "Raytracing in one weekend". Link: https://github.com/RayTracing/raytracing.github.io.
I only completed the first book. I might return and do the others later, but right now i want to move on to other stuff.

I tried to make it faster by adding my own multithreading aswell which went very well.

## tl:dr How it works 
I start by creating a vector of all the objects in the world that can be hit and I give each object a material. It can either have a glass-, metal-, or diffuse, each of which implement the Material-trait. 

Then the basic idea is, we send out rays, from a virtual camera. The rays hit objects in the world, bounce off it in some manner, and hit other stuff. With each bounce, the color of the ray changes a bit due to the object's material. After a ray has bounced around a number of times, it's color is recorded, and the pixel from which the ray was originally sent out from, gets that color. The recorded colors get stored in a vector, and then turned into an image.

## The Rays
The rays are just a 3d line. They have an origin and a direction.

```math
\vec{dir} = \begin{pmatrix} a\\b\\c \end{pmatrix}
```
```math
\begin{pmatrix} x\\y\\z \end{pmatrix} = \begin{pmatrix} x_0\\y_0\\z_0 \end{pmatrix} + t \cdot \vec{dir}
```

## The objects
The only objects that my raytracer can currently work with are spheres. In a fancier raytracer you would probably work with triangles, because more shapes are possible with triangles. But for me, spheres it is. Spheres are defined from their placement in 3d space aswell as their radius. 

```math
P = \begin{pmatrix} x\\y\\z \end{pmatrix}
```
```math
(x-C_x)^2 + (y-C_y)^2 + (z-C_z)^2 = r^2
```

## Ray-Object intersection
Let's rewrite the function for the ray so it looks like this: 
```math 
P(t) = \vec{A} + t \cdot \vec{B}
```
Where *A* is the ray-origin and *B* is the direction.

If our ray and a sphere intersects, that must mean that there is some value, t, for P(t) that satisfies the sphere equation. From this knowledge we can isolate a function for ray-sphere intersections.
Lets call the point P and the center of the sphere C. We can now rewrite the formula for a sphere like this:
```math
(\vec{P} - \vec{C})^2 = r^2
```
We can insert the formula for a ray into this equation instead of the P, the point, since P is the same for both.
```math
((\vec{A} + t \cdot \vec{B}) - C)^2 = r^2
```
