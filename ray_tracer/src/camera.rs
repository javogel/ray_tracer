use crate::{
    canvas::*,
    matrix::{identity, Matrix},
    ray::{ray, Ray},
    transforms::translation,
    tuple::{point, Tuple},
    world::World,
};

pub struct Camera {
    pub hsize: i16,
    pub vsize: i16,
    pub field_of_view: f32,
    pub half_width: f32,
    pub half_height: f32,
    pub pixel_size: f32,
    pub transform: Matrix<f32>,
}

impl Camera {
    pub fn new(hsize: i16, vsize: i16, field_of_view: f32) -> Self {
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
        let x_offset = (x as f32 + 0.5) * self.pixel_size;
        let y_offset = (y as f32 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_inverse = self.transform.inverse().unwrap();
        let pixel = transform_inverse.clone() * point(world_x, world_y, -1.);
        let origin = transform_inverse * point(0., 0., 0.);

        let direction = (pixel - origin).normalize();

        return ray(origin, direction);
    }

    fn compute_fields(hsize: i16, vsize: i16, field_of_view: f32) -> (f32, f32, f32) {
        let half_view = (field_of_view / 2.).tan();
        let aspect = (hsize as f32) / (vsize as f32);
        let half_width;
        let half_height;
        if aspect >= 1. {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.) / (hsize as f32);

        (half_width, half_height, pixel_size)
    }
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix<f32> {
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

pub fn camera(hsize: i16, vsize: i16, field_of_view: f32) -> Camera {
    Camera::new(hsize, vsize, field_of_view)
}

pub fn render(camera: Camera, world: World) -> Canvas {
    let mut image = canvas(camera.hsize as usize, camera.vsize as usize);
    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = &camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray);

            match image.write_pixel(x as usize, y as usize, color) {
                Err(e) => println!("error rendering to pixel: {:?}", e),
                _ => (),
            }
        }
    }
    return image;
}
