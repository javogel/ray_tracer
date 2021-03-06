use crate::{
    canvas::*,
    matrix::{identity, Matrix},
    ray::{ray, Ray},
    transforms::translation,
    tuple::{point, Tuple},
    utils::RECURSION_DEPTH,
    world::World,
};
use rayon::prelude::*;

pub struct Camera {
    pub hsize: i16,
    pub vsize: i16,
    pub field_of_view: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
    pub transform: Matrix<f64>,
}

impl Camera {
    pub fn new(hsize: i16, vsize: i16, field_of_view: f64) -> Self {
        let (half_width, half_height, pixel_size) =
            Self::compute_fields(hsize, vsize, field_of_view);

        Self {
            hsize,
            vsize,
            field_of_view,
            pixel_size,
            half_width,
            half_height,
            transform: identity(),
        }
    }

    pub fn ray_for_pixel(&self, x: i16, y: i16) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_inverse = self.transform.inverse().unwrap();
        let pixel = transform_inverse.clone() * point(world_x, world_y, -1.);
        let origin = transform_inverse * point(0., 0., 0.);

        let direction = (pixel - origin).normalize();

        return ray(origin, direction);
    }

    fn compute_fields(hsize: i16, vsize: i16, field_of_view: f64) -> (f64, f64, f64) {
        let half_view = (field_of_view / 2.).tan();
        let aspect = (hsize as f64) / (vsize as f64);
        let half_width;
        let half_height;
        if aspect >= 1. {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.) / (hsize as f64);

        (half_width, half_height, pixel_size)
    }
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix<f64> {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);

    let orientation = Matrix::from(vec![
        vec![left.x, left.y, left.z, 0.],
        vec![true_up.x, true_up.y, true_up.z, 0.],
        vec![-forward.x, -forward.y, -forward.z, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    return orientation * translation(-from.x, -from.y, -from.z);
}

pub fn camera(hsize: i16, vsize: i16, field_of_view: f64) -> Camera {
    Camera::new(hsize, vsize, field_of_view)
}

pub fn render(camera: Camera, world: World) -> Canvas {
    let mut image = canvas(camera.hsize as usize, camera.vsize as usize);
    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = &camera.ray_for_pixel(x, y);

            let color = world.color_at(&ray, RECURSION_DEPTH);

            match image.write_pixel(x as usize, y as usize, color) {
                Err(e) => println!("error rendering to pixel: {:?}", e),
                _ => (),
            }
        }
    }
    return image;
}

pub fn render_parallelized(camera: Camera, world: World) -> Canvas {
    let (width, height) = (camera.hsize as usize, camera.vsize as usize);
    let mut pixels = vec![0.; width * height * 3];
    let bands: Vec<(usize, &mut [f64])> = pixels.chunks_mut(width * 3).enumerate().collect();

    bands.into_par_iter().for_each(|(i, band)| {
        // for y in 0..1 {
        for x in 0..width {
            let ray = &camera.ray_for_pixel(x as i16, i as i16);
            let color = world.color_at(&ray, RECURSION_DEPTH);

            let index = (3 * x) as usize;
            band[index] = color.r;
            band[index + 1] = color.g;
            band[index + 2] = color.b;
        }
        // }
    });

    let mut image = canvas(camera.hsize as usize, camera.vsize as usize);
    image.pixels = pixels;
    return image;
}
