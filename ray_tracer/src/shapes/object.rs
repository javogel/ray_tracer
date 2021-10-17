use crate::{material::Material, matrix::Matrix, tuple::Tuple};

use super::sphere::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        self.uuid() == other.uuid()
    }
}

impl Object {
    pub fn uuid(&self) -> Uuid {
        return match self {
            Self::Sphere(a) => a.uuid,
        };
    }

    pub fn material(&self) -> &Material {
        return match self {
            Self::Sphere(a) => &a.material,
        };
    }

    pub fn set_material(&mut self, material: Material) {
        return match self {
            Self::Sphere(s) => s.material = material,
        };
    }

    pub fn transform(&self) -> &Matrix<f64> {
        return match self {
            Self::Sphere(a) => &a.transform,
        };
    }

    pub fn set_transform(&mut self, transform: Matrix<f64>) {
        return match self {
            Self::Sphere(s) => s.transform = transform,
        };
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        return match self {
            Self::Sphere(a) => a.normal_at(p),
        };
    }

    pub fn new_sphere() -> Self {
        let c = default_sphere();
        Self::Sphere(c)
    }
}
