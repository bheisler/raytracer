#[macro_use]
extern crate serde_derive;
extern crate image;

mod scene;
mod rendering;
mod vector;
mod matrix;
mod point;

use scene::Scene;
use image::DynamicImage;

use rendering::Ray;

/*pub fn render(scene: &Scene) -> DynamicImage {
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray: Ray = Ray::create_prime(x, y, scene);

        }
    }
}*/


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
