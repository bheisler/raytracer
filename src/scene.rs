use point::Point;
use vector::Vector3;
use rendering::{Intersectable, Ray};
use std::ops::Mul;

#[derive(Deserialize, Debug, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl Color {
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}
impl<'a> Mul for &'a Color {
    type Output = Color;

    fn mul(self, other: &'a Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

#[derive(Deserialize, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

#[derive(Deserialize, Debug)]
pub struct Plane {
    pub origin: Point,
    #[serde(deserialize_with="Vector3::deserialize_normalized")]
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
}


#[derive(Deserialize, Debug)]
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

    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.albedo,
            Element::Plane(ref p) => p.albedo,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Light {
    #[serde(deserialize_with="Vector3::deserialize_normalized")]
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub light: Light,
    pub shadow_bias: f64,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,

    //Prevent outside code from constructing this; should use the new method and check the distance.
    _secret: (),
}
impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Intersection {
            distance: distance,
            element: element,
            _secret: (),
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
