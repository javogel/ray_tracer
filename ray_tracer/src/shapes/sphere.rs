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
    radius: f32,
    pub transform: Matrix<f32>,
}

impl Sphere {
    pub fn new(center: Tuple, radius: f32) -> Self {
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
    pub fn set_transform(&mut self, transform: Matrix<f32>) {
        self.transform = transform;
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let transform_inverse = self.transform.inverse().unwrap();
        let object_point = transform_inverse.clone() * p;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let world_normal = transform_inverse.transpose() * object_normal;
        let mut normal = world_normal.normalize();
        normal.w = TupleType::Vector; // see pg. 82
        normal
    }
}

pub fn sphere(center: Tuple, radius: f32) -> Sphere {
    Sphere::new(center, radius)
}
