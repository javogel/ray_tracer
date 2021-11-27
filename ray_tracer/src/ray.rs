use crate::{matrix::Matrix, shapes::object::*, tuple::*, utils::EPSILON};
use std::{fmt, ops::Index};

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

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

    pub fn intersect<'a>(&self, object: &'a Object) -> Intersect<'a> {
        object.intersect(self)
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

impl<'a> Clone for Intersection<'a> {
    fn clone(&self) -> Self {
        Self {
            t: self.t.clone(),
            object: self.object,
        }
    }
}

impl<'a> fmt::Display for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Intersection: {}; Uuid: {}", self.t, self.object.uuid)
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
