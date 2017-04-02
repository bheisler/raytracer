#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

pub mod scene;
pub mod vector;
pub mod point;
mod rendering;
mod matrix;

use scene::Scene;
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};

use rendering::{Ray, cast_ray};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
        }
    }
    image
}

pub fn render_into(scene: &Scene, image: &mut ImageBuffer<Rgba<u8>, &mut [u8]>) {
    for y in 0..scene.height {
        for x in 0..scene.width {
            let ray = Ray::create_prime(x, y, scene);
            image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
        }
    }
}
