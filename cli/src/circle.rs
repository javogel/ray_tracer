#![allow(dead_code)]
use ray_tracer::canvas::{canvas, Dimensions, ImageType};
use ray_tracer::color::color;
use ray_tracer::light::{lighting, point_light, PointLight};
use ray_tracer::ray::{intersect, ray};
use ray_tracer::shapes::sphere::*;
use ray_tracer::tuple::*;
use ray_tracer::{material::*, transforms::*};
use std::f32::consts::PI;

pub fn draw_chapter_4_exercise() {
    let mut c = canvas(300, 300);
    let Dimensions { width, height } = c.dimensions;

    for i in 0..12 {
        let p = point(0., 0.2, 0.)
            .rotate_z((i as f32) * PI / 6.)
            .translate(0.5, 0.5, 0.);

        let x = (width as f32 * p.x) as usize;
        let y = (height as f32 * p.y) as usize;
        c.write_pixel(x, y, color(0.9, 0.9, 0.9)).unwrap();
    }

    c.save(ImageType::PPM, "chapter4");
}

pub fn create_sphere() -> Sphere {
    let mut shape = sphere(point(0., 0., 0.), 1.);
    shape.material = material();
    shape.material.color = color(0.1, 0.8, 0.9);
    shape.transform = shearing(1.0, 0., 0., 0., 0., 0.) * scaling(0.5, 1., 1.) * rotation_y(PI);
    return shape;
}

pub fn create_light() -> PointLight {
    let position = point(-10., 10., -10.);
    let light_color = color(1., 1., 1.);
    return point_light(position, light_color);
}
pub fn draw_chapter_5_exercise() {
    let ray_origin = point(0., 0., -5.);
    let wall_z = 10.;

    let wall_size = 7.;

    let canvas_pixels = 500;
    let pixel_size = wall_size / (canvas_pixels as f32);

    let half = wall_size / 2.;

    let mut c = canvas(canvas_pixels, canvas_pixels);
    // let shape_color = color(1.0, 0., 0.);
    let shape = create_sphere();
    let light = create_light();
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f32);

        for x in 0..canvas_pixels {
            let world_x = half - pixel_size * (x as f32);
            let position = point(world_x, world_y, wall_z);

            let r = ray(ray_origin, normalize(position - ray_origin));
            let xs = intersect(&r, &shape);

            match xs.hit() {
                Some(_) => {
                    let point = r.position(xs.locations[0].t);
                    let normal = shape.normal_at(point);
                    let eye = -r.direction;
                    let color = lighting(&shape.material, &light, point, eye, normal);
                    c.write_pixel(x, y, color).unwrap()
                }
                None => (),
            }
        }
    }

    c.save(ImageType::PPM, "chapter6-random-color");
}
