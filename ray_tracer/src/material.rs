use crate::color::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

fn default_material() -> Material {
    Material {
        color: color(1., 1., 1.),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.,
    }
}
pub fn material() -> Material {
    default_material()
}
