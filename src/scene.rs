#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}
