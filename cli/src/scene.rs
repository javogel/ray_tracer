use rand::prelude::*;
use std::f32::consts::PI;

use ray_tracer::{
    camera::{camera, render, view_transform},
    color::color,
    light::point_light,
    material::material,
    shapes::{object::Object, sphere::*},
    transforms::{rotation_x, rotation_y, scaling, translation},
    tuple::*,
    world::world,
};

pub fn draw_chapter_7_exercise() {
    let mut rng = thread_rng();

    let mut floor = default_sphere();
    floor.transform = scaling(10., 0.01, 10.);
    floor.material.color = color(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let mut left_wall = default_sphere();
    left_wall.transform = translation(0., 0., 5.)
        * rotation_y(-PI / 4.)
        * rotation_x(PI / 2.)
        * scaling(10., 0.01, 10.);
    left_wall.material = floor.material.clone();

    let mut right_wall = default_sphere();
    right_wall.transform = translation(0., 0., 5.)
        * rotation_y(PI / 4.)
        * rotation_x(PI / 2.)
        * scaling(10., 0.01, 10.);
    right_wall.material = floor.material.clone();

    let mut middle = default_sphere();
    middle.transform = translation(-1. * rng.gen::<f32>(), 1. * rng.gen::<f32>(), 0.5);
    middle.material = material();
    middle.material.color = color(
        1. * rng.gen::<f32>(),
        1. * rng.gen::<f32>(),
        1. * rng.gen::<f32>(),
    );
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = default_sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5 * rng.gen::<f32>(), 0.5);
    right.material = material();
    right.material.color = color(
        0.5 * rng.gen::<f32>(),
        1. * rng.gen::<f32>(),
        1. * rng.gen::<f32>(),
    );
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = default_sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material = material();
    left.material.color = color(
        1.0 * rng.gen::<f32>(),
        1. * rng.gen::<f32>(),
        1. * rng.gen::<f32>(),
    );
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let w = world(
        point_light(point(-10., 10., -10.), color(1., 1., 1.)),
        vec![
            Object::Sphere(floor),
            Object::Sphere(left_wall),
            Object::Sphere(right_wall),
            Object::Sphere(middle),
            Object::Sphere(left),
            Object::Sphere(right),
        ],
    );
    let mut camera = camera(600, 300, PI / 3.);

    camera.transform = view_transform(point(0., 1.5, -5.), point(0., 1., 0.), vector(0., 1., 0.));

    let c = render(camera, w);

    c.save(ray_tracer::canvas::ImageType::PPM, "chapter8");
}
