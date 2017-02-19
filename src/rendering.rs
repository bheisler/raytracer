use point::Point;
use vector::Vector3;
use scene::{Scene, Sphere};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let screen_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) *
                       fov_adjustment;
        let screen_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                    x: screen_x,
                    y: screen_y,
                    z: -1.0,
                }
                .normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let center = Point {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector3 = center - ray.origin;
        //Use l as a hypotenuse and find the length-squared of the adjacent side
        let tca = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (tca * tca)
        let d2 = l.dot(&l) - (tca * tca);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}
