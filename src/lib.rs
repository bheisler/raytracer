#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

pub mod scene;
mod rendering;
mod vector;
mod matrix;
mod point;

use scene::{Scene, Color, Intersection};
use image::{DynamicImage, GenericImage, Rgba, Pixel};

use rendering::{Ray, Intersectable};

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);

    let mut color = Color {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    };
    for light in &scene.lights {
        let direction_to_light = -light.direction;

        let shadow_ray = Ray {
            origin: hit_point + (direction_to_light * scene.shadow_bias),
            direction: direction_to_light,
        };
        let in_light = scene.trace(&shadow_ray).is_none();

        let light_intensity = if in_light { light.intensity } else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) *
                          light_intensity;
        let light_reflected = intersection.element.albedo() / std::f32::consts::PI;

        let light_color = light.color * light_power * light_reflected;
        color = color + (*intersection.element.color() * light_color);
    }

    color.clamp()
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            let intersection = scene.trace(&ray);
            let color = intersection.map(|i| to_rgba(&get_color(scene, &ray, &i)))
                .unwrap_or(black);
            image.put_pixel(x, y, color);
        }
    }
    image
}

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels((color.red * 255.0) as u8,
                        (color.green * 255.0) as u8,
                        (color.blue * 255.0) as u8,
                        0)
}
