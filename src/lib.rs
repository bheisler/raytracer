#[macro_use]
extern crate serde_derive;
extern crate image;

pub mod scene;
mod rendering;
mod vector;
mod matrix;
mod point;

use scene::{Scene, Color};
use image::{DynamicImage, GenericImage, Rgba, Pixel};

use rendering::{Ray, Intersectable};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, to_rgba(&scene.sphere.color))
            } else {
                image.put_pixel(x, y, black);
            }
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

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
        },
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

#[cfg(test)]
mod tests {
    use scene::{Scene, Color, Sphere};
    use image::{DynamicImage, GenericImage};
    use super::render;

    #[test]
    fn test_can_render_scene() {
        let scene = Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            sphere: Sphere {
                x: 0.0,
                y: 0.0,
                z: -5.0,
                radius: 1.0,
                color: Color {
                    red: 100,
                    green: 255,
                    blue: 100,
                },
            },
        };

        let img: DynamicImage = render(&scene);
        assert_eq!(scene.width, img.width());
        assert_eq!(scene.height, img.height());
    }
}
