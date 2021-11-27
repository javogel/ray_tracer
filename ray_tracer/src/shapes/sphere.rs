use crate::{
    ray::{intersection, Intersect, Ray},
    tuple::*,
};

use super::object::Object;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Tuple,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Tuple, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn local_normal_at(&self, object_point: Tuple) -> Tuple {
        return object_point - point(0.0, 0.0, 0.0);
    }

    fn calc_sphere(&self, r: &Ray) -> (f64, f64, f64) {
        let sphere_to_ray = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let b = dot(r.direction, sphere_to_ray) * 2.;
        let c = dot(sphere_to_ray, sphere_to_ray) - 1.;

        (a, b, c)
    }

    pub fn local_intersect<'a>(&self, parent_object: &'a Object, r: &Ray) -> Intersect<'a> {
        let (a, b, c) = self.calc_sphere(r);

        let discriminant = b * b - a * c * 4.;

        let locations = if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (a * 2.);
            let t2 = (-b + discriminant.sqrt()) / (a * 2.);

            let i1 = intersection(t1, parent_object);
            let i2 = intersection(t2, parent_object);

            if i1.t < i2.t {
                vec![i1, i2]
            } else {
                vec![i2, i1]
            }
        };

        Intersect { locations }
    }
}

pub fn sphere(center: Tuple, radius: f64) -> Sphere {
    Sphere::new(center, radius)
}

pub fn default_sphere() -> Sphere {
    Sphere::new(point(0., 0., 0.), 1.)
}
