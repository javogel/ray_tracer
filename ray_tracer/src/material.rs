use std::fmt;

use crate::{color::*, pattern::Pattern};

#[derive(Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    // pub pattern: Option<Box<dyn Pattern>>,
}

pub fn default_material() -> Material {
    Material {
        color: color(1., 1., 1.),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.,
        // pattern: None,
    }
}
pub fn material() -> Material {
    default_material()
}
