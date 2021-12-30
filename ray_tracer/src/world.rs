use crate::{
    color::{color, Color},
    light::{is_shadowed, lighting, point_light, PointLight},
    material::Material,
    ray::{ray, Intersect, Intersection, Ray},
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

    pub fn shade_hit<'a>(&self, c: &'a PreparedComputations, remaining: u8) -> Color {
        let shadowed = is_shadowed(self, c.over_point);

        let surface = lighting(
            &c.object.material,
            &c.object,
            &self.light,
            c.point,
            c.eyev,
            c.normalv,
            shadowed,
        );

        let reflected = self.reflected_color(c, remaining);
        let refracted = self.refracted_color(c, remaining);

        return surface + reflected + refracted;
    }

    pub fn color_at(&self, r: &Ray, remaining_depth: u8) -> Color {
        let intersections = self.intersect(r);

        return match intersections.hit() {
            Some(i) => {
                let comps = prepare_computations(&i, r, &intersections);
                self.shade_hit(&comps, remaining_depth)
            }
            None => color(0., 0., 0.),
        };
    }

    pub fn reflected_color(&self, comps: &PreparedComputations, remaining: u8) -> Color {
        if remaining <= 0 || comps.object.material.reflective == 0. {
            return color(0., 0., 0.);
        }

        let reflected_ray = ray(comps.over_point, comps.reflectv);

        let reflected_color = self.color_at(&reflected_ray, remaining - 1);

        return reflected_color * comps.object.material.reflective;
    }

    pub fn refracted_color(&self, comps: &PreparedComputations, remaining: u8) -> Color {
        if comps.object.material.transparency == 0. || remaining <= 0 {
            return color(0., 0., 0.);
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
        if sin2_t > 1. {
            return color(0., 0., 0.);
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

        let refracted_ray = ray(comps.under_point, direction);

        return self.color_at(&refracted_ray, remaining - 1) * comps.object.material.transparency;
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
        reflective: 0.0,
        transparency: 0.,
        refractive_index: 1.,
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
    pub reflectv: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub under_point: Tuple,
}

pub fn prepare_computations<'a>(
    i: &'a Intersection,
    r: &Ray,
    xs: &Intersect,
) -> PreparedComputations<'a> {
    let point = r.position(i.t);
    let mut normalv = i.object.normal_at(point);
    let eyev = -r.direction;
    let mut inside = false;

    if normalv.dot(eyev) < 0. {
        normalv = -normalv;
        inside = true
    }
    let over_point = point + normalv * EPSILON;
    let under_point = point - normalv * EPSILON;

    let reflectv = r.direction.reflect(normalv);

    let mut containers: Vec<&Intersection> = vec![];
    let mut n1 = 0.0;
    let mut n2 = 0.0;

    for intersect in xs.locations.iter() {
        if i == intersect {
            if containers.is_empty() {
                n1 = 1.0
            } else {
                n1 = containers.last().unwrap().object.material.refractive_index
            }
        }

        if let Some(index) = containers.iter().position(|value| *value == intersect) {
            containers.swap_remove(index);
        } else {
            containers.push(intersect)
        }

        if i == intersect {
            if containers.is_empty() {
                n2 = 1.0
            } else {
                n2 = containers.last().unwrap().object.material.refractive_index
            }
            break;
        }
    }

    PreparedComputations {
        object: &i.object,
        t: i.t,
        point,
        eyev,
        normalv,
        inside,
        over_point,
        reflectv,
        n1,
        n2,
        under_point,
    }
}
