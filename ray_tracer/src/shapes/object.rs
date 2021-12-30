use std::fmt::Debug;

use super::{plane::*, sphere::*};
use crate::{
    material::{default_material, Material},
    matrix::{identity, Matrix},
    ray::{Intersect, Ray},
    tuple::{Tuple, TupleType},
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Clone)]
pub struct Object {
    pub uuid: Uuid,
    pub shape: Shape,
    pub transform: Matrix<f64>,
    pub material: Material,
}

impl Object {
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn set_transform(&mut self, transform: Matrix<f64>) {
        self.transform = transform;
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let transform_inverse = self.transform.inverse().unwrap();
        let object_point = transform_inverse.clone() * p;
        let object_normal = match &self.shape {
            Shape::Sphere(shape) => shape.local_normal_at(object_point),
            Shape::Plane(shape) => shape.local_normal_at(object_point),
            // _ => panic!("Shape's local_normal_at has not been implemented"),
        };
        let mut world_normal = transform_inverse.transpose() * object_normal;
        world_normal.w = TupleType::Vector; // see pg. 82
        return world_normal.normalize();
    }

    pub fn intersect(&self, r: &Ray) -> Intersect {
        let ray = r.transform(&self.transform.inverse().unwrap());
        return match &self.shape {
            Shape::Sphere(a) => a.local_intersect(self, &ray),
            Shape::Plane(a) => a.local_intersect(self, &ray),
            // _ => panic!("Shape's local_intersect has not been implemented"),
        };
    }

    fn new(shape: Shape) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            shape,
            material: default_material(),
            transform: identity(),
        }
    }

    pub fn new_sphere() -> Self {
        let s = default_sphere();
        Self::new(Shape::Sphere(s))
    }

    pub fn new_glass_sphere() -> Self {
        let s = default_sphere();
        let mut obj = Self::new(Shape::Sphere(s));
        obj.material.transparency = 1.;
        obj.material.refractive_index = 1.5;
        return obj;
    }

    pub fn new_plane() -> Self {
        let p = plane();
        Self::new(Shape::Plane(p))
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        self.uuid == other.uuid
    }
}
