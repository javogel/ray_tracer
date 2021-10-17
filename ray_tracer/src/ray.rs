// #![allow(dead_code)]

use crate::{
    matrix::Matrix,
    shapes::{object::*, sphere::*},
    tuple::*,
    utils::EPSILON,
};
use std::{ops::Index, vec};
// use uuid::Uuid;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

pub struct Intersect<'a> {
    pub locations: Vec<Intersection<'a>>,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    fn calc_sphere(&self, s: &Sphere) -> (f64, f64, f64) {
        let r = self.transform(&s.transform.inverse().unwrap());
        let sphere_to_ray = r.origin - s.center;
        let a = dot(r.direction, r.direction);
        let b = dot(r.direction, sphere_to_ray) * 2.;
        let c = dot(sphere_to_ray, sphere_to_ray) - 1.;

        (a, b, c)
    }

    pub fn intersect<'a>(&self, s: &'a Object) -> Intersect<'a> {
        let (a, b, c) = match s {
            Object::Sphere(s) => self.calc_sphere(s),
        };

        let discriminant = b * b - a * c * 4.;

        let locations = if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (a * 2.);
            let t2 = (-b + discriminant.sqrt()) / (a * 2.);

            let i1 = intersection(t1, s);
            let i2 = intersection(t2, s);

            if i1.t < i2.t {
                vec![i1, i2]
            } else {
                vec![i2, i1]
            }
        };

        Intersect { locations }
    }

    pub fn transform(&self, transformation: &Matrix<f64>) -> Self {
        Self {
            direction: transformation * self.direction,
            origin: transformation * self.origin,
        }
    }
}

impl<'a> Intersect<'a> {
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

impl<'a> Index<usize> for Intersect<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        return match self.locations.len() {
            0 => panic!("No intersections exist"),
            a if index <= a - 1 => &self.locations[index],
            _ => panic!("Invalid index into intersection"),
        };
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Intersection) -> bool {
        if self.t - other.t > EPSILON {
            return false;
        }
        self.object == other.object
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray::new(origin, direction)
}

pub fn intersect<'a>(r: &'a Ray, s: &'a Object) -> Intersect<'a> {
    r.intersect(s)
}

pub fn intersection<'a>(t: f64, object: &'a Object) -> Intersection<'a> {
    Intersection { t, object }
}

pub fn intersections(locations: Vec<Intersection>) -> Intersect {
    Intersect { locations }
}
