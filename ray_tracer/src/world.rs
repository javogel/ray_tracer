use crate::{
    color::{color, Color},
    light::{is_shadowed, lighting, point_light, PointLight},
    material::Material,
    ray::{Intersect, Intersection, Ray},
    shapes::object::Object,
    transforms::scaling,
    tuple::{point, Tuple},
    utils::EPSILON,
};

pub struct World {
    pub objects: Vec<Object>,
    pub light: PointLight,
}

impl World {
    pub fn intersect(&self, r: &Ray) -> Intersect {
        let mut locations = vec![];
        for obj in &self.objects {
            let mut i = r.intersect(obj);
            if i.count() > 0 {
                locations.append(&mut i.locations)
            }
        }
        locations.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        return Intersect { locations };
    }

    pub fn shade_hit<'a>(&self, c: &'a PreparedComputations) -> Color {
        let shadowed = is_shadowed(self, c.over_point);

        return lighting(
            &c.object.material,
            &c.object,
            &self.light,
            c.point,
            c.eyev,
            c.normalv,
            shadowed,
        );
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        let intersections = self.intersect(r);

        return match intersections.hit() {
            Some(i) => {
                let comps = prepare_computations(&i, r);
                self.shade_hit(&comps)
            }
            None => color(0., 0., 0.),
        };
    }
}
pub fn world(light: PointLight, objects: Vec<Object>) -> World {
    World { light, objects }
}

pub fn default_world() -> World {
    let origin = point(-10., 10., -10.);
    let light_color = color(1., 1., 1.);
    let light = point_light(origin, light_color);

    let mut sphere1 = Object::new_sphere();
    sphere1.set_material(Material {
        color: color(0.8, 1.0, 0.6),
        diffuse: 0.7,
        specular: 0.2,
        ambient: sphere1.material.ambient,
        shininess: sphere1.material.shininess,
        pattern: None,
    });
    let mut sphere2 = Object::new_sphere();
    sphere2.set_transform(scaling(0.5, 0.5, 0.5));

    world(light, vec![sphere1, sphere2])
}

pub struct PreparedComputations<'a> {
    pub object: &'a Object,
    pub t: f64,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

pub fn prepare_computations<'a>(i: &'a Intersection, r: &Ray) -> PreparedComputations<'a> {
    let point = r.position(i.t);
    let mut normalv = i.object.normal_at(point);
    let eyev = -r.direction;
    let mut inside = false;

    if normalv.dot(eyev) < 0. {
        normalv = -normalv;
        inside = true
    }
    let over_point = point + normalv * EPSILON;

    PreparedComputations {
        object: &i.object,
        t: i.t,
        point,
        eyev,
        normalv,
        inside,
        over_point,
    }
}
