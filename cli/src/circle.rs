use ray_tracer::canvas::{Canvas, Dimensions};
use ray_tracer::color::color;
use ray_tracer::transforms::*;
use ray_tracer::tuple::*;
use std::f32::consts::PI;

pub fn draw_on(canvas: &mut Canvas) {
    let Dimensions { width, height } = canvas.dimensions;

    for i in 0..12 {
        let p = point(0., 0.2, 0.)
            .rotate_z((i as f32) * PI / 6.)
            .translate(0.5, 0.5, 0.);

        let x = (width as f32 * p.x) as usize;
        let y = (height as f32 * p.y) as usize;
        canvas.write_pixel(x, y, color(0.9, 0.9, 0.9)).unwrap();
    }
}