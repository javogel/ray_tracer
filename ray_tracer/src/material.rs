use crate::{color::*, pattern::Pattern};

pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Box<dyn Pattern>>,
    pub reflective: f64,
}

pub fn default_material() -> Material {
    Material {
        color: color(1., 1., 1.),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.,
        pattern: None,
        reflective: 0.0,
    }
}
pub fn material() -> Material {
    default_material()
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Self {
            pattern: None,
            ..*self
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        return self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.shininess == other.shininess
            && self.specular == other.specular;
    }
}
