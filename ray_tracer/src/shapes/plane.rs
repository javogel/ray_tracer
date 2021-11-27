use crate::{
    ray::{intersection, Intersect, Ray},
    tuple::*,
    utils::EPSILON,
};

use super::object::Object;

#[derive(Debug, Clone)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn local_normal_at(&self, _: Tuple) -> Tuple {
        return vector(0., 1., 0.);
    }

    pub fn local_intersect<'a>(&self, parent_object: &'a Object, ray: &Ray) -> Intersect<'a> {
        if ray.direction.y.abs() < EPSILON {
            return Intersect { locations: vec![] };
        }

        let t = -ray.origin.y / ray.direction.y;

        return Intersect {
            locations: vec![intersection(t, parent_object)],
        };
    }
}

pub fn plane() -> Plane {
    Plane::new()
}
