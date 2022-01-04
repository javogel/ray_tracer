#![allow(dead_code)]
// use rand::prelude::*;
use std::f64::consts::PI;

use ray_tracer::{
    camera::{camera, render, render_parallelized, view_transform},
    color::color,
    light::point_light,
    material::{default_material, material},
    pattern::*,
    shapes::object::Object,
    transforms::*,
    tuple::*,
    world::world,
};

pub fn draw_chapter_7_exercise() {
    let mut floor = Object::new_sphere();
    floor.transform = scaling(10., 0.01, 10.);
    floor.material.color = color(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let mut left_wall = Object::new_sphere();
    left_wall.transform = translation(0., 0., 5.)
        * rotation_y(-PI / 4.)
        * rotation_x(PI / 2.)
        * scaling(10., 0.01, 10.);
    left_wall.material = floor.material.clone();

    let mut right_wall = Object::new_sphere();
    right_wall.transform = translation(0., 0., 5.)
        * rotation_y(PI / 4.)
        * rotation_x(PI / 2.)
        * scaling(10., 0.01, 10.);
    right_wall.material = floor.material.clone();

    let mut middle = Object::new_sphere();
    middle.transform = translation(-0.5, 1., 0.5);
    middle.material = material();
    middle.material.color = color(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Object::new_sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material = material();
    right.material.color = color(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Object::new_sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material = material();
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let w = world(
        point_light(point(-10., 10., -10.), color(1., 1., 1.)),
        vec![floor, left_wall, right_wall, middle, left, right],
    );
    let mut camera = camera(1000, 500, PI / 3.);

    camera.transform = view_transform(point(0., 1.5, -5.), point(0., 1., 0.), vector(0., 1., 0.));

    let c = render(camera, w);

    c.save(ray_tracer::canvas::ImageType::PPM, "chapter8");
}

pub fn draw_chapter_9_exercise() {
    let mut floor = Object::new_plane();
    // floor.transform = scaling(10., 10., 10.);
    // floor.material.color = color(1., 0.9, 0.9);
    // floor.material.specular = 0.;
    {
        let mut pattern = gradient_pattern(color(0.5, 0.2, 0.9), color(0.33, 0.44, 0.99));
        pattern.transform = translation(0., 0., 5.) * scaling(10., 10., 10.) * rotation_y(PI / 2.);
        floor.material.pattern = Some(Box::new(pattern));
        floor.material.reflective = 0.0;
    }

    let mut back_wall = Object::new_plane();
    back_wall.transform = translation(0., 0., 7.) * rotation_x(PI / 2.);
    // back_wall.material.color = color(0.9, 0.7, 0.6);
    // back_wall.material.specular = 0.2;
    back_wall.material.reflective = 0.1;
    {
        let mut pattern = stripe_pattern(color(0.91, 0.4, 0.2), color(0.2, 0.54, 0.80));
        pattern.transform = scaling(1.0, 1.0, 1.) * rotation_x(PI);
        back_wall.material.pattern = Some(Box::new(pattern));
    }

    let mut front_wall = Object::new_plane();
    front_wall.transform = translation(0., 0., -15.) * rotation_x(PI / 2.);
    front_wall.material.color = color(0.5, 0.2, 0.9);
    front_wall.material.reflective = 0.;

    let mut left_wall = Object::new_plane();
    left_wall.transform = translation(-15., 0., 0.) * rotation_z(PI / 2.);
    left_wall.material.color = color(1., 0.9, 0.3);
    left_wall.material.specular = 0.2;
    left_wall.material.reflective = 0.;

    let mut right_wall = Object::new_plane();
    right_wall.transform = translation(15., 0., 0.) * rotation_z(PI / 2.);
    right_wall.material.color = color(1., 0.9, 0.3);
    right_wall.material.specular = 0.5;
    right_wall.material.reflective = 0.;

    let mut ceiling = Object::new_plane();
    ceiling.transform = translation(0., 10., 0.);
    ceiling.material.color = color(0.5, 0.8, 0.9);
    ceiling.material.specular = 0.9;
    ceiling.material.reflective = 0.;

    let mut middle = Object::new_sphere();
    middle.transform = translation(-0.5, 0.5, -2.5) * scaling(0.50, 0.50, 0.50);
    middle.material = default_material();
    middle.material.color = color(0., 0., 0.);
    // middle.material.diffuse = 0.9;
    // middle.material.specular = 0.9;
    middle.material.reflective = 1.;
    middle.material.transparency = 1.0;
    middle.material.refractive_index = 1.5;

    let mut right = Object::new_sphere();
    right.transform = translation(1.5, 0.5, 0.5) * scaling(0.5, 0.5, 0.5);
    right.material = material();
    right.material.color = color(0.56, 0.3, 0.8);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.reflective = 0.5;

    let mut left = Object::new_sphere();
    left.transform = translation(-1.5, 0.33, 0.75) * scaling(0.33, 0.33, 0.33);
    left.material = material();
    left.material.color = color(1.0, 0.8, 0.4);
    left.material.diffuse = 0.7;
    left.material.specular = 0.8;
    left.material.reflective = 0.5;

    let w = world(
        point_light(point(-5., 5., -10.), color(1., 1., 1.)),
        vec![
            floor, back_wall, front_wall, left_wall, right_wall, ceiling, middle, left, right,
        ],
    );
    let mut camera = camera(1200, 1200, PI / 3.);

    camera.transform = view_transform(point(0., 1.5, -5.), point(0., 1., 0.), vector(0., 1., 0.));

    // let c = render(camera, w);
    let c = render_parallelized(camera, w);

    c.save(
        ray_tracer::canvas::ImageType::PPM,
        "chapter11-refraction-exploration",
    );
}
