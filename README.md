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
### The basic idea
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
```math
((\vec{A} + t \cdot \vec{B}) - C) \cdot ((\vec{A} + t \cdot \vec{B}) - C)= r^2
```
If we then seperate the elements with t, we get familiar looking binomial equation: 
```math 
(a+b)^2 = a^2 + b^2 + 2ab
```
```math
(t \vec{B} + ( \vec{A} - C)) \cdot (t  \vec{B} + ( \vec{A} - C))= r^2
```
Calculate it all out we get:
```math
t^2B^2 + 2tB \cdot (A - C) + (A-C)^2 - r^2 = 0
```

Now this looks a lot like quadratic formula where t is our only unknown variable. Therefore we should be able to solve it from the quadratic theorem:
```math
t_{1/2} = {-b \pm \sqrt{b^2 - 4ac} \over 2a}
```
```math
a = B^2
```
```math
b = 2B \cdot (A - C)
```
```math
c = (A-C)^2 - r^2
```
If we insert these, we get a nice equation for us to solve in the program to figure out if a ray has intersected a sphere:
```math
D = b^2 - 4ac
```
```math
D = (2B \cdot (A - C))^2 - 4(B^2)((A-C)^2 - r^2)
```
```math
t_{1/2} = {-2B \cdot (A - C) \pm \sqrt{D} \over 2B^2}
```
This is great and works. We can even use the discriminant to figure out if the the ray intersects the sphere at all, before we calculate the exact point:
![image](https://github.com/MthsBttgr/raytracer/assets/94607744/01357e0e-0964-49af-85f4-7690e30e5fda)

### Better idea
Now, people smarter than I, figured out how to make the formula even more concice:
let's say that b = 2h.
```math
b = 2h
```
then we get the following formula:
```math
t_{1/2} = {-2h \pm \sqrt{(2h)^2 - 4ac} \over 2a}
```
```math
= {-2h \pm 2\sqrt{(h)^2 - ac} \over 2a}
```
```math
= {-h \pm \sqrt{(h)^2 - ac} \over a}
```
If we insert the actual values we get the following formula:
```math
a = B^2
```
```math
h = B \cdot (A - C)
```
```math
c = (A-C)^2 - r^2
```
```math
= {-B \cdot (A - C) \pm \sqrt{(B \cdot (A - C))^2 - (B^2)((A-C)^2 - r^2)} \over B^2}
```
From this formula we get the following code for ray-sphere intersections:
```rust 
    /// Calculates if a ray hits the sphere, and returns a hitrecord if it is hit.
    /// Else it returns none.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // using quadratic formula to calculate intersections
        let oc = r.origin() - self.center;                           // oc = (A - C), A is the origin of the ray ie. the camera, C is the center of the sphere
        let a = r.direction().length_squared();                      // a = B^2, B is the direction-vector of of the ray
        let half_b = oc.dot_product(&r.direction());                 // half_b = h = oc * B = (A - C) * B
        let c = oc.length_squared() - self.radius.powi(2);           // c = oc^2 - r^2 = (A - C)^2 - r^2

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // calculates the sqrt once, and saves the value
        let discriminant_sqrt = discriminant.sqrt();

        // detect the nearest hit:
        let mut hit = (-half_b - discriminant_sqrt) / a;
        if hit <= t_min || t_max <= hit {
            // adding the discriminant_sqrt is always further away
            hit = (-half_b + discriminant_sqrt) / a;
            if hit <= t_min || t_max <= hit {
                return None;
            }
        }

        // Constructs the hitrecord
        let point = r.at(hit);
        let record = HitRecord::new(
            point,
            hit,
            (point - self.center) / self.radius,
            &self.material,
        );

        Some(record)
    }
```

## Hitrecord - what is that?
In the code above you might notice the construction of a hitrecord. A hitrecord is used to store data about each "hit" with an object. So every time there is an intersection between a ray and an object, a hitrecord is created. The hitrecord stores information such as the point where the intersection happened, the distance from the point to the camera, the normal of the point on the surface, and what material that object has. 

### What is a normal?
A normal is an unit vector that stands perpendicular on a surface. From the normal we can figure out the angle the ray hit the object at. This is useful when calculation how a ray should bounce off the surface of an object, or figuring out if the ray is hitting the backside of a surface. 

## Let's talk about the camera
But before we can even view the scene we need a place to view it from: The camera. The camera emulates a real world camera, the only major difference being that the virtual camera sends out rays and a real one collects them. It is in the camera where we decide all the settings of the picture. How big the resolution is, aspect ratio, focus (called defocus_blur in the code), placement, and more. 

### Defining the camera 
This section might be slightly confusing and maybe a bit all over the place. There are a bunch of things we need to simulate a virtual camera, and many of them interact to create other things we need. I will use a bunch of terms and do my best to explain them, but it might be hard to follow none the less.

First of all the camera needs a placement and a point to look at. It also needs a rotation, but currently im just saying that it is always horizontal. From this information we can generate 3 vectors that we use to define our camera in virtual space.
```rust 
        // Camera positioning
        let look_from = Point3::from_xyz(0, 0, 1);
        let look_at = Point3::from_xyz(0, 0, 0);
        let vup = Vec3::from_xyz(0, 1, 0); // Camera - relative up direction

        // Camera basic vectors
        let w = (look_from - look_at).unit_vec();
        let u = vup.cross_product(&w).unit_vec();
        let v = u.cross_product(&w);
```
![image](https://github.com/MthsBttgr/raytracer/assets/94607744/0ae326e8-88ed-4643-aae8-f2f9afc42104)
(Technically, at this stage the virtual camera is a perfect unit square, and not a rectangle as shown in the illustration. However, it was easier to illustrate the u, v, and w vectors from a rectangular virtual camera)

The u and v vectors are used to define our "image plane". This is where we will later lay out all the pixels. The w vector points towards the camera origin. The distance from the camera origin to the image plane is called the focal distance. In my program the focal distance is the same as the focus distance, which is the distance from the camera sensor too where everything is in perfect focus. That means the image plane is where everything is in perfect focus. In a real world camera, the focal distance and focus distance are obviously very different, because the lense can't extend to the object it is focusing on, but in the virtual world we aren't limited by what is physically impossible.

Now we need to "shape" the the camera. For this we need an aspect ratio ie. the ratio between the width of the camera and the height, and the field of view. I have gone with a classic 16:9 aspect ratio, though this is easy to change. Field of view, fov, describes the visual angle from edge to edge. Basically how much we can see. A small fov makes it so we can only see a small part of the scene, giving the effect that the camera is zoomed in, and a large fov gives the reverse effect. The fov is different depending on whether you are measuring from the top and bottom edges of the view plane or the left and right edges. I am using vertical fov, vfov, in the program ie. the angle between the top and bottom edges. Now, through a bit of math we can "shape" the camera:
![image](https://github.com/MthsBttgr/raytracer/assets/94607744/e4d66b7e-87b8-4a1e-8149-820475b62b05)
h is equal to half the height of the camera. h is easy to calculate with just a bit of trigonomitri:
```math 
h = \tan{({vfov \over 2})} \cdot focusDistance
```
```math 
cameraHeight = 2h
```
```math 
cameraWidth = aspectRatio \cdot cameraHeight
```
And that ends up looking like this in the code:
```rust
        // image dimensions
        let aspect_ratio = 16.0 / 9.0;
        let img_width = 400;                                                // Horizontal pixel count
        let img_height = (img_width as f64 / aspect_ratio) as i64;          // Vertical pixel count


        // Viewport dimensions:
        let vfov = 90.0;
        let theta = Camera::degrees_to_radians(vfov);                      // Changing the angle from degrees to radians
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;                    // The virtual camera is called the viewport in the code
        let viewport_width = viewport_height * aspect_ratio;

```
(I also define the image resolution here ie. how many horizontal pixels and how many vertical pixels)

Now, if we take these values and multiply by our camera vectors, v and u, we get the shape of the camera we want. We can use this information to lay out the pixel grid. Think of each pixel as a tiny square, and a bunch of these squares are laid out over the image plane. Through some simple math we find the distance between the center of each pixel, aswell as the location of the top-left pixel, pixel_00_loc, which will be our starting point when rendering the picture, which i will explain what is in a bit. 
```rust 
        let viewport_u = u * viewport_width;
        let viewport_v = v * viewport_height;
        let pixel_delta_u = viewport_u / img_width as f64;
        let pixel_delta_v = viewport_v / img_height as f64;
        let upper_left = look_from - (w * focus_distance) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = upper_left + (pixel_delta_v + pixel_delta_u) * 0.5;
```
![image](https://github.com/MthsBttgr/raytracer/assets/94607744/94df31ee-8097-4156-b34b-9a0de4216668)

There are a few more variables that are stored in the camera, the samples per pixel and the maximum number of light bounces. These are fairly simpel, but I think it will be easier to explain it in the context of rendering the image, where they are used. The camera also stores some variables used for bluring the parts of the image that is out of focus. I think that is easier to explain after explaining how a picture is renderes aswell.

## Rendering
The idea behind rendering the image is simple. It is the process of actually sending out rays, recording their color, and creating the image file. I will explain this process iterativly, slowly getting more complex as more elements are added to this process. First let's look at some code:
```rust
    pub fn render<T: Hitable>(&self, world: &T, file: &mut BufWriter<File>) {
        //writing header to image file
        file.write_all(format!("P3\n{} {}\n255\n", self.img_width, self.img_height).as_bytes())
            .expect("couldnt write header");

        // loop through each pixel
        for y in 0..self.img_height {
            for x in 0..self.img_width {
                // create a ray that goes from the camera origin through the given pixel center
                let r: Ray = self.get_ray(x as f64, y as f64);
                // Simulate light bouncing through the scene and record the color
                let pixel_color: Color = Camera::ray_color(&r, world, self.max_light_bounces);    // the world parameter is just a list of every object in the scene that the rays can collide with.

                // Write the pixel color to the image file
                file.write_all(format!("\n{}", pixel_color.write_color(self.samples_pr_pixel as f64)).as_bytes(),)
                    .expect("couldnt write all");
            }
        }
    }
```

...to be continued...
