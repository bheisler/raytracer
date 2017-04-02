use point::Point;
use vector::Vector3;
use scene::{Scene, Element, Sphere, Plane, Color, Intersection, SurfaceType};
use std::f32;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width >= scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) *
                       fov_adjustment;
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

    pub fn create_reflection(normal: Vector3,
                             incident: Vector3,
                             intersection: Point,
                             bias: f64)
                             -> Ray {
        Ray {
            origin: intersection + (normal * bias),
            direction: incident - (2.0 * incident.dot(&normal) * normal),
        }
    }

    pub fn create_transmission(normal: Vector3,
                               incident: Vector3,
                               intersection: Point,
                               bias: f64,
                               index: f32)
                               -> Option<Ray> {
        let mut ref_n = normal;
        let mut eta_t = index as f64;
        let mut eta_i = 1.0;
        let mut i_dot_n = incident.dot(&normal);
        if i_dot_n < 0.0 {
            //Outside the surface
            i_dot_n = -i_dot_n;
        } else {
            //Inside the surface; invert the normal and swap the indices of refraction
            ref_n = -normal;
            eta_i = eta_t;
            eta_t = 1.0;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Ray {
                origin: intersection + (ref_n * -bias),
                direction: (incident + i_dot_n * ref_n) * eta - ref_n * k.sqrt(),
            })
        }
    }
}

#[derive(Debug)]
pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;

    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        match *self {
            Element::Sphere(ref s) => s.texture_coords(hit_point),
            Element::Plane(ref p) => p.texture_coords(hit_point),
        }
    }
}
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector3 = self.center - ray.origin;
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
            None
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let hit_vec = *hit_point - self.center;
        TextureCoords {
            x: (1.0 + (hit_vec.z.atan2(hit_vec.x) as f32) / f32::consts::PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() as f32 / f32::consts::PI,
        }
    }
}
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);
        let hit_vec = *hit_point - self.origin;

        TextureCoords {
            x: hit_vec.dot(&x_axis) as f32,
            y: hit_vec.dot(&y_axis) as f32,
        }
    }
}

const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

fn shade_diffuse(scene: &Scene,
                 element: &Element,
                 hit_point: Point,
                 surface_normal: Vector3)
                 -> Color {
    let texture_coords = element.texture_coords(&hit_point);
    let mut color = BLACK;
    for light in &scene.lights {
        let direction_to_light = light.direction_from(&hit_point);

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none() ||
                       shadow_intersection.unwrap().distance > light.distance(&hit_point);

        let light_intensity = if in_light {
            light.intensity(&hit_point)
        } else {
            0.0
        };
        let material = element.material();
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) *
                          light_intensity;
        let light_reflected = material.albedo / f32::consts::PI;

        let light_color = light.color() * light_power * light_reflected;
        color = color + (material.coloration.color(&texture_coords) * light_color);
    }
    color.clamp()
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit = ray.origin + (ray.direction * intersection.distance);
    let normal = intersection.element.surface_normal(&hit);

    let material = intersection.element.material();
    match material.surface {
        SurfaceType::Diffuse => shade_diffuse(scene, intersection.element, hit, normal),
        SurfaceType::Reflective { reflectivity } => {
            let mut color = shade_diffuse(scene, intersection.element, hit, normal);
            let reflection_ray =
                Ray::create_reflection(normal, ray.direction, hit, scene.shadow_bias);
            color = color * (1.0 - reflectivity);
            color = color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);
            color
        }
        SurfaceType::Refractive { index, transparency } => {
            let mut refraction_color = BLACK;
            let kr = fresnel(ray.direction, normal, index) as f32;
            let surface_color = material.coloration
                .color(&intersection.element.texture_coords(&hit));

            if kr < 1.0 {
                let transmission_ray =
                    Ray::create_transmission(normal, ray.direction, hit, scene.shadow_bias, index)
                        .unwrap();
                refraction_color = cast_ray(scene, &transmission_ray, depth + 1);
            }

            let reflection_ray =
                Ray::create_reflection(normal, ray.direction, hit, scene.shadow_bias);
            let reflection_color = cast_ray(scene, &reflection_ray, depth + 1);
            let mut color = reflection_color * kr + refraction_color * (1.0 - kr);
            color = color * transparency * surface_color;
            color
        }
    }
}

fn fresnel(incident: Vector3, normal: Vector3, index: f32) -> f64 {
    let i_dot_n = incident.dot(&normal);
    let mut eta_i = 1.0;
    let mut eta_t = index as f64;
    if i_dot_n > 0.0 {
        eta_i = eta_t;
        eta_t = 1.0;
    }

    let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
    if sin_t > 1.0 {
        //Total internal reflection
        return 1.0;
    } else {
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        let cos_i = cos_t.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        return (r_s * r_s + r_p * r_p) / 2.0;
    }
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= scene.max_recursion_depth {
        return BLACK;
    }

    let intersection = scene.trace(&ray);
    intersection.map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(BLACK)
}
