mod circle;
mod projectile;
use ray_tracer::canvas::*;

fn main() {
    // // Chapter 2 Exercise
    // let mut c = canvas(900, 500);
    // projectile::run_simulation(&mut c);
    // c.save(ImageType::PPM, "image2");

    // Chapter 4 Exercise
    let mut c = canvas(300, 300);
    circle::draw_on(&mut c);
    c.save(ImageType::PPM, "chapter4");
}
