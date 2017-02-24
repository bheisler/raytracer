use point::Point;
use rendering::{Intersectable, Ray};

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub spheres: Vec<Sphere>,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Sphere,

    //Prevent outside code from constructing this; should use the new method and check the distance.
    _secret: (),
}
impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Sphere) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Intersection {
            distance: distance,
            object: object,
            _secret: (),
        }
    }
}
impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.spheres
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
