#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

pub mod scene;
mod rendering;
mod vector;
mod matrix;
mod point;

use scene::Scene;
use image::{DynamicImage, GenericImage};

use rendering::{Ray, cast_ray};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            image.put_pixel(x, y, cast_ray(scene, &ray).to_rgba());
        }
    }
    image
}
