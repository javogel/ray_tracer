#![allow(dead_code)]

use crate::{
    matrix::{identity, Matrix},
    tuple::*,
    utils::EPSILON,
};
use std::{ops::Index, vec};
use uuid::Uuid;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f32,
    pub object_uuid: Uuid,
}

pub struct Intersect {
    locations: Vec<Intersection>,
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub uuid: Uuid,
    center: Tuple,
    radius: f32,
    pub transform: Matrix<f32>,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn intersect(&self, s: &Sphere) -> Intersect {
        let r = self.transform(&s.transform.inverse().unwrap());
        let sphere_to_ray = r.origin - s.center;
        let a = dot(r.direction, r.direction);
        let b = dot(r.direction, sphere_to_ray) * 2.;
        let c = dot(sphere_to_ray, sphere_to_ray) - 1.;

        let discriminant = b * b - a * c * 4.;

        let locations = if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (a * 2.);
            let t2 = (-b + discriminant.sqrt()) / (a * 2.);

            let i1 = intersection(t1, s.uuid);
            let i2 = intersection(t2, s.uuid);

            if i1.t < i2.t {
                vec![i1, i2]
            } else {
                vec![i2, i1]
            }
        };

        Intersect { locations }
    }

    pub fn transform(&self, transformation: &Matrix<f32>) -> Self {
        Self {
            direction: transformation * self.direction,
            origin: transformation * self.origin,
        }
    }
}

impl Intersect {
    pub fn count(&self) -> usize {
        self.locations.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        return self
            .locations
            .clone()
            .into_iter()
            .filter(|x| x.t > 0.)
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }
}

impl Index<usize> for Intersect {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        return match self.locations.len() {
            0 => panic!("No intersections exist"),
            a if index <= a - 1 => &self.locations[index],
            _ => panic!("Invalid index into intersection"),
        };
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t - other.t < EPSILON && self.object_uuid == other.object_uuid
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray::new(origin, direction)
}

impl Sphere {
    pub fn new(center: Tuple, radius: f32) -> Self {
        let uuid = Uuid::new_v4();
        let transform = identity();
        Sphere {
            uuid,
            center,
            radius,
            transform,
        }
    }
    pub fn set_transform(&mut self, transform: Matrix<f32>) {
        self.transform = transform;
    }
}

pub fn sphere(center: Tuple, radius: f32) -> Sphere {
    Sphere::new(center, radius)
}

pub fn intersect(r: Ray, s: &Sphere) -> Intersect {
    r.intersect(s)
}

pub fn intersection(t: f32, object_uuid: Uuid) -> Intersection {
    Intersection { t, object_uuid }
}

pub fn intersections(locations: Vec<Intersection>) -> Intersect {
    Intersect { locations }
}
