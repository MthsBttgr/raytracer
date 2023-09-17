use rand::{rngs::ThreadRng, Rng};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Vec3 = Point3;

/// Struct containing 3 f64 values and vector functions
#[derive(Debug, Default, Clone, Copy)]
pub struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    /// Creates a new point at (0,0,0)
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Creates a new point at the given xyz coordinates
    pub fn from_xyz(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// returns x-coordinate
    pub fn x(&self) -> f64 {
        self.x
    }

    /// returns y-coordinate
    pub fn y(&self) -> f64 {
        self.y
    }

    /// returns z-coordinate
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Returns the distance between this point and another
    pub fn distance(&self, other: &Point3) -> f64 {
        let dx = self.x - other.x();
        let dy = self.y - other.y();
        let dz = self.z - other.z();

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// length of the vector between the point and (0,0,0)
    pub fn length(&self) -> f64 {
        self.distance(&Point3::new())
    }

    /// length of the vector between the point and (0,0,0) squared
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// the dotproduct between this vector and another
    pub fn dot_product(&self, other: &Point3) -> f64 {
        self.x * other.x() + self.y * other.y() + self.z * other.z()
    }

    /// the cross product between this vector and another
    pub fn cross_product(&self, other: &Point3) -> Point3 {
        Point3::from_xyz(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// returns the unitvector
    pub fn unit_vec(&self) -> Vec3 {
        *self / self.length()
    }

    /// creates a random vec with random values from min to (but not including) max
    pub fn random_vec_from_to(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    /// Gets a random vec inside a sphere with the radius of one
    pub fn random_vec_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_vec_from_to(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    /// Gets a random unit vector
    pub fn random_unit_vec() -> Vec3 {
        Point3::random_vec_in_unit_sphere().unit_vec()
    }

    /// Checks if Self is close to a null vec
    pub fn near_zero(&self) -> bool {
        let num = 1e-10;
        (self.x < num) && (self.y < num) && (self.z < num)
    }

    /// Generates a random 2 dimensional vector (z = 0) with a lenght less than one
    pub fn random_in_unit_circle(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let p = Vec3::from_xyz(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Self {
        Point3 {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Sub for Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Self {
        Point3 {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Sub<f64> for Point3 {
    type Output = Point3;

    fn sub(self, other: f64) -> Self {
        Point3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Self {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Point3> for Point3 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Self {
        Point3 {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, other: f64) -> Self {
        Point3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<Point3> for Point3 {
    type Output = Point3;

    fn div(self, other: Point3) -> Self {
        Point3 {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Div<f64> for Point3 {
    type Output = Point3;

    fn div(self, other: f64) -> Self {
        Point3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Into<Color> for Point3 {
    fn into(self) -> Color {
        Color::from_rgb(self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    /// creates new color that is completely black: r = 0, g = 0, b = 0.
    pub fn new() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    /// creates a new color from rgb values.
    /// The values should be between 0 and 1
    pub fn from_rgb(r: impl Into<f64>, g: impl Into<f64>, b: impl Into<f64>) -> Color {
        Color {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }

    /// Returns a string containing the color values of the color.
    /// The values are scaled by 255.999 and rounded down.
    /// The string returned looks like this: "{r} {g} {b}", so just the color values and no "\n" or anything
    pub fn write_color(&self, samples_pr_pixel: f64) -> String {
        let mut r = self.r;
        let mut g = self.g;
        let mut b = self.b;

        let scale = 1.0 / samples_pr_pixel;

        r *= scale;
        g *= scale;
        b *= scale;

        return format!(
            "{} {} {}",
            (256.0 * clamp(r.sqrt(), 0.0, 0.999)) as i32,
            (256.0 * clamp(g.sqrt(), 0.0, 0.999)) as i32,
            (256.0 * clamp(b.sqrt(), 0.0, 0.999)) as i32
        );
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self {
        Color {
            r: self.r + other.r(),
            g: self.g + other.g(),
            b: self.b + other.b(),
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self {
        Color {
            r: self.r - other.r(),
            g: self.g - other.g(),
            b: self.b - other.b(),
        }
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Self {
        Color {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self {
        Color {
            r: self.r * other.r(),
            g: self.g * other.g(),
            b: self.b * other.b(),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Div<Color> for Color {
    type Output = Color;

    fn div(self, other: Color) -> Self {
        Color {
            r: self.r / other.r(),
            g: self.g / other.g(),
            b: self.b / other.b(),
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Self {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

trait IsBetween {
    fn is_between(&self, a: f64, b: f64) -> bool;
}

impl IsBetween for f64 {
    fn is_between(&self, a: f64, b: f64) -> bool {
        a <= *self && *self <= b
    }
}

#[inline]
fn clamp(num: f64, min: f64, max: f64) -> f64 {
    if num < min {
        return min;
    }
    if num > max {
        return max;
    }
    num
}
