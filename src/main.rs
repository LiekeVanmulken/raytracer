// https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/
extern crate image;
//extern crate cgmath;

use std::ops::Sub;
use image::{Rgba, GenericImage, Pixel};
use std::fs::File;


fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        spheres: [
            Sphere {
                center: Point {
                    x: 1.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
            },
            Sphere {
                center: Point {
                    x: 3.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                },
            }
        ],
    };

    let image = render(&scene);

    let ref mut fout = File::create("test.png").unwrap();
    image.save(fout, image::PNG).unwrap();
}


pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3::from_one(0.0)
    }

    pub fn from_one(v: f64) -> Vector3 {
        Vector3 { x: v, y: v, z: v }
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Vector3 {
        let inv_len = self.length().recip();
        Vector3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, other: Vector3) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Point> for Vector3 {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        other - self
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point::from_one(0.0)
    }

    pub fn from_one(v: f64) -> Point {
        Point { x: v, y: v, z: v }
    }
}

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}


pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub spheres: [Sphere; 2],
}


pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);

        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
                .normalize(),
        }
    }
}


pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let a = Point { x: self.center.x, y: self.center.y, z: self.center.z };
        let b = Point { x: ray.origin.x, y: ray.origin.y, z: ray.origin.z };

        let l: Vector3 = a - b;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
}

pub fn to_rgba(color: &Color) -> Rgba<u8> {
    return Rgba::from_channels((color.red * 255.0) as u8,
                               (color.green * 255.0) as u8,
                               (color.blue * 255.0) as u8, 255);
}


pub fn render(scene: &Scene) -> image::DynamicImage {
    let mut image = image::DynamicImage::new_rgb8(scene.width, scene.height);
    let black = image::Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let mut is_set: bool = false;
            let mut shortest: f64 = 300000.0;
            let mut count: usize = 0;

            for s_count in 0..scene.spheres.len() {
                let sphere: &Sphere = &scene.spheres[s_count];
                let ray: Ray = Ray::create_prime(x, y, scene);
                if x == 0 && y == 0 {
                    println!("x : {0} y : {1} z : {2}", ray.origin.x, ray.origin.y, ray.origin.z);
                    println!("x : {0} y : {1} z : {2}", ray.direction.x, ray.direction.y, ray.direction.z);
                }
                let a = sphere.intersect(&ray);
                if a.is_some() { // todo : add the closest sphere with the value
//                println!("intersects");
                    image.put_pixel(x, y, to_rgba(&sphere.color));
                    is_set = true;
                    if a.unwrap() < shortest {
                        count = s_count;
                        shortest = a.unwrap();
                    }
                }
            }
            if is_set {
                let sphere: &Sphere = &scene.spheres[count];
                image.put_pixel(x, y, to_rgba(&sphere.color));
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

//
//pub struct Intersection<'a> {
//    pub distance: f64,
//    pub object: &'a Sphere,
//}
//impl<'a> Intersection<'a> {
//    pub fn new<'b>(distance: f64, object: &'b Sphere) -> Intersection<'b> {
//        return Intersection();
//    }
//}
//impl Scene {
//    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
//        self.spheres
//            .iter()
//            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
//            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
//    }
//}


pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color,
        }
    }
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);

        if denom > 1e-6 {
            let v = Point { x: self.origin.x, y: self.origin.y, z: self.origin.z }
                -
                Point { x: ray.origin.x, y: ray.origin.y, z: ray.origin.z };
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}