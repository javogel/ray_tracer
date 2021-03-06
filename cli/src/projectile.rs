#![allow(dead_code)]

use ray_tracer::canvas::{canvas, ImageType};
use ray_tracer::color::color;
use ray_tracer::tuple::*;
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}
fn tick(p: &mut Projectile, e: &Environment) {
    p.position = p.position + p.velocity;
    p.velocity = p.velocity + e.gravity + e.wind;
}

pub fn run_simulation() {
    let mut c = canvas(900, 500);
    let mut projectile = Projectile {
        position: point(0., 1., 0.),
        velocity: normalize(vector(1., 1.8, 0.)) * 11.25,
    };
    let environment = Environment {
        gravity: vector(0., -0.1, 0.),
        wind: vector(-0.01, 0., 0.),
    };
    while projectile.position.y > 0. {
        tick(&mut projectile, &environment);
        let x = projectile.position.x as usize;
        let y = c.height() - projectile.position.y as usize;
        c.write_pixel(x, y, color(0.9, 0.5, 0.)).unwrap();
    }

    c.save(ImageType::PPM, "chapter2");
}
