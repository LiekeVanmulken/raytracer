pub mod data {
    pub struct Point {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    pub struct Color {
        pub red: f32,
        pub green: f32,
        pub blue: f32,
    }

    pub enum Element {
        Sphere(Sphere),
        Plane(Plane),
    }

    pub struct Sphere {
        pub center: Point,
        pub radius: f64,
        pub color: Color,
    }

    pub struct Plane {
        pub origin: Point,
        pub normal: Vector3,
        pub color: Color,
    }

    pub struct Light {
        pub direction: Vector3,
        pub color: Color,
        pub intensity: f32,
    }

    pub trait Intersectable {
        fn intersect(&self, ray: &Ray) -> Option<f64>;
    }

    pub struct Scene {
        pub width: u32,
        pub height: u32,
        pub fov: f64,
        pub elements: Vec<Element>,
        pub light: Light,
    }

    pub struct Ray {
        pub origin: Point,
        pub direction: Vector3,
    }
}