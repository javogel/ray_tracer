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
    let mut projectile = Projectile {
        position: point(1., 1., 0.),
        velocity: normalize(vector(1., 1., 0.)),
    };
    let environment = Environment {
        gravity: vector(0., -0.1, 0.),
        wind: vector(-0.01, 0., 0.),
    };
    while projectile.position.y > 0. {
        println!("{}", &projectile.position);
        tick(&mut projectile, &environment);
    }
}
