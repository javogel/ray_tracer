mod projectile;
use ray_tracer::canvas::*;

fn main() {
    // projectile::run_simulation()
    // // canvas::run()
    // ray_tracer::canvas::run();

    let mut c = canvas(900, 500);
    projectile::run_simulation(&mut c);
    // render::save_as_ppm(&c)
    c.save(ImageType::PPM);
}
