mod circle;
mod projectile;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // // Chapter 2 Exercise
    // projectile::draw_chapter_2_exercise();

    // Chapter 4 Exercise
    // circle::draw_chapter_4_exercise();

    // Chapter 5 Exercise
    circle::draw_chapter_5_exercise();
    let duration = start.elapsed();
    println!("Rendering duration: {:?}", duration);
}
