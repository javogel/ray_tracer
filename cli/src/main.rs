mod circle;
mod projectile;
mod scene;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // // Chapter 2 Exercise
    // projectile::draw_chapter_2_exercise();

    // Chapter 4 Exercise
    // circle::draw_chapter_4_exercise();

    // Chapter 5 Exercise
    // circle::draw_chapter_5_exercise();

    // Chapter 7 Exercise
    scene::draw_chapter_7_exercise();
    let duration = start.elapsed();
    println!("Rendering duration: {:?}", duration);
}
