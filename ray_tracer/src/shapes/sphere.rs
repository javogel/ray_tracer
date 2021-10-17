use uuid::Uuid;

use crate::{
    material::*,
    matrix::{identity, Matrix},
    tuple::*,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub uuid: Uuid,
    pub center: Tuple,
    pub material: Material,
    radius: f64,
    pub transform: Matrix<f64>,
}

impl Sphere {
    pub fn new(center: Tuple, radius: f64) -> Self {
        let uuid = Uuid::new_v4();
        let transform = identity();
        Sphere {
            uuid,
            center,
            material: material(),
            radius,
            transform,
        }
    }
    pub fn set_transform(&mut self, transform: Matrix<f64>) {
        self.transform = transform;
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let transform_inverse = self.transform.inverse().unwrap();
        let object_point = transform_inverse.clone() * p;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = transform_inverse.transpose() * object_normal;
        world_normal.w = TupleType::Vector; // see pg. 82
        return world_normal.normalize();
    }
}

pub fn sphere(center: Tuple, radius: f64) -> Sphere {
    Sphere::new(center, radius)
}

pub fn default_sphere() -> Sphere {
    Sphere::new(point(0., 0., 0.), 1.)
}
