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

#[repr(C)]
#[derive(Debug)]
pub struct ViewBlock {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn render(block: &ViewBlock, scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(block.width, block.height);
    for y in 0..block.height {
        for x in 0..block.width {
            let ray = Ray::create_prime(x + block.x, y + block.y, scene);
            image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
        }
    }
    image
}

pub fn render_into(block: &ViewBlock,
                   scene: &Scene,
                   image: &mut ImageBuffer<Rgba<u8>, &mut [u8]>) {
    for y in 0..block.height {
        for x in 0..block.width {
            let ray = Ray::create_prime(x + block.x, y + block.y, scene);
            image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
        }
    }
}
