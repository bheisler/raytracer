extern crate raytracer;
extern crate image;

use raytracer::point::Point;
use raytracer::vector::Vector3;
use raytracer::scene::*;
use std::path::PathBuf;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

#[no_mangle]
pub extern "C" fn scene_new(width: u32,
                            height: u32,
                            fov: f64,
                            shadow_bias: f64,
                            max_recursion_depth: u32)
                            -> *mut Scene {
    let scene = Box::new(Scene {
        width: width,
        height: height,
        fov: fov,

        elements: vec![],
        lights: vec![],

        shadow_bias: shadow_bias,
        max_recursion_depth: max_recursion_depth,
    });
    Box::into_raw(scene)
}

#[no_mangle]
pub extern "C" fn scene_add_sphere(scene: *mut Scene,
                                   center: *const Point,
                                   radius: f64,
                                   material: *const CMaterial) {
    if scene.is_null() || center.is_null() || material.is_null() {
        return;
    }
    let mut scene = unsafe { Box::from_raw(scene) };

    if let Some(rust_material) = unsafe { (&*material) }.to_rust() {
        let sphere = Sphere {
            center: unsafe { &*center }.clone(),
            radius: radius,
            material: rust_material,
        };
        let mut scene_ref = &mut *scene;
        scene_ref.elements.push(Element::Sphere(sphere));
    }

    //Don't free the scene
    Box::into_raw(scene);
}

#[no_mangle]
pub extern "C" fn scene_add_plane(scene: *mut Scene,
                                  origin: *const Point,
                                  normal: *const Vector3,
                                  material: *const CMaterial) {
    if scene.is_null() || origin.is_null() || normal.is_null() || material.is_null() {
        return;
    }
    let mut scene = unsafe { Box::from_raw(scene) };

    if let Some(rust_material) = unsafe { (&*material) }.to_rust() {
        let plane = Plane {
            origin: unsafe { (&*origin) }.clone(),
            normal: unsafe { (&*normal) }.normalize(),
            material: rust_material,
        };
        let mut scene_ref = &mut *scene;
        scene_ref.elements.push(Element::Plane(plane));
    }

    //Don't free the scene
    Box::into_raw(scene);
}

#[no_mangle]
pub extern "C" fn scene_add_spherical_light(scene: *mut Scene,
                                            position: *const Point,
                                            color: *const Color,
                                            intensity: f32) {
    if scene.is_null() || position.is_null() || color.is_null() {
        return;
    }
    let mut scene = unsafe { Box::from_raw(scene) };
    let light = SphericalLight {
        position: unsafe { &*position }.clone(),
        color: unsafe { &*color }.clone(),
        intensity: intensity,
    };
    {
        let mut scene_ref = &mut *scene;
        scene_ref.lights.push(Light::Spherical(light));
    }

    //Don't free the scene
    Box::into_raw(scene);
}

#[no_mangle]
pub extern "C" fn scene_add_directional_light(scene: *mut Scene,
                                              direction: *const Vector3,
                                              color: *const Color,
                                              intensity: f32) {
    if scene.is_null() || direction.is_null() || color.is_null() {
        return;
    }
    let mut scene = unsafe { Box::from_raw(scene) };
    let light = DirectionalLight {
        direction: unsafe { &*direction }.normalize(),
        color: unsafe { &*color }.clone(),
        intensity: intensity,
    };
    {
        let mut scene_ref = &mut *scene;
        scene_ref.lights.push(Light::Directional(light));
    }

    //Don't free the scene
    Box::into_raw(scene);
}

#[no_mangle]
pub extern "C" fn scene_render(scene: *mut Scene, buffer: *mut u8, length: usize) {
    if scene.is_null() || buffer.is_null() {
        return;
    }
    let scene = unsafe { Box::from_raw(scene) };
    let buffer = unsafe { slice::from_raw_parts_mut(buffer, length) };

    if let Some(mut image) = image::ImageBuffer::from_raw(scene.width, scene.height, buffer) {
        raytracer::render_into(&*scene, &mut image);
    }

    //Don't free the scene
    Box::into_raw(scene);
}

#[no_mangle]
pub extern "C" fn scene_free(ptr: *mut Scene) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

//Create a second
pub enum CColoration {
    CColor { color: Color },
    CTexture { path: PathBuf },
}
impl CColoration {
    fn to_rust(&self) -> Option<Coloration> {
        match *self {
            CColoration::CColor { ref color } => Some(Coloration::Color(color.clone())),
            CColoration::CTexture { ref path } => {
                if let Ok(texture) = image::open(path) {
                    Some(Coloration::Texture(texture))
                } else {
                    None
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn coloration_color_new(red: f32, green: f32, blue: f32) -> *mut CColoration {
    let coloration = Box::new(CColoration::CColor {
        color: Color {
            red: red,
            green: green,
            blue: blue,
        },
    });
    Box::into_raw(coloration)
}

#[no_mangle]
pub extern "C" fn coloration_texture_new(s: *const c_char) -> *mut CColoration {
    if s.is_null() {
        return ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(s) };
    if let Ok(str) = c_str.to_str() {
        let coloration = Box::new(CColoration::CTexture { path: PathBuf::from(str) });
        Box::into_raw(coloration)
    } else {
        return ptr::null_mut();
    }
}

#[no_mangle]
pub extern "C" fn coloration_free(ptr: *mut CColoration) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn surfacetype_diffuse_new() -> *mut SurfaceType {
    let surface = Box::new(SurfaceType::Diffuse);
    Box::into_raw(surface)
}

#[no_mangle]
pub extern "C" fn surfacetype_reflective_new(reflectivity: f32) -> *mut SurfaceType {
    let surface = Box::new(SurfaceType::Reflective { reflectivity: reflectivity });
    Box::into_raw(surface)
}

#[no_mangle]
pub extern "C" fn surfacetype_refractive_new(index: f32, transparency: f32) -> *mut SurfaceType {
    let surface = Box::new(SurfaceType::Refractive {
        index: index,
        transparency: transparency,
    });
    Box::into_raw(surface)
}

#[no_mangle]
pub extern "C" fn surfacetype_free(ptr: *mut SurfaceType) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[repr(C)]
pub struct CMaterial {
    coloration: *const CColoration,
    surface: *const SurfaceType,
    albedo: f32,
}
impl CMaterial {
    pub fn to_rust(&self) -> Option<Material> {
        if self.coloration.is_null() || self.surface.is_null() {
            return None;
        }
        if let Some(coloration) = unsafe { &*self.coloration }.to_rust() {
            Some(Material {
                coloration: coloration,
                albedo: self.albedo,
                surface: unsafe { &*self.surface }.clone(),
            })
        } else {
            None
        }
    }
}
