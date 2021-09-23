use std::f32::consts::PI;

use ray_tracer::{
    camera::{camera, render, view_transform},
    color::color,
    matrix::{identity, Matrix},
    transforms::{rotation_y, scaling, translation},
    tuple::{point, vector},
    world::default_world,
};

#[test]
fn view_transormation_for_default_orientation() {
    let from = point(0., 0., 0.);
    let to = point(0., 0., -1.);
    let up = vector(0., 1., 0.);

    let t = view_transform(from, to, up);
    assert_eq!(t, identity())
}

#[test]
fn view_transform_looking_in_positive_z_direction() {
    let from = point(0., 0., 0.);
    let to = point(0., 0., 1.);
    let up = vector(0., 1., 0.);

    let t = view_transform(from, to, up);
    assert_eq!(t, scaling(-1., 1., -1.))
}

#[test]
fn view_transform_moves_the_world() {
    let from = point(0., 0., 8.);
    let to = point(0., 0., 0.);
    let up = vector(0., 1., 0.);

    let t = view_transform(from, to, up);
    assert_eq!(t, translation(0., 0., 8.))
}

#[test]
fn arbitrary_view_transform() {
    let from = point(1., 3., 2.);
    let to = point(4., -2., 8.);
    let up = vector(1., 1., 0.);

    let t = view_transform(from, to, up);
    let expected_transform = Matrix::from(vec![
        vec![-0.50709, 0.50709, 0.67612, -2.36643],
        vec![0.76772, 0.60609, 0.12122, -2.82843],
        vec![-0.35857, 0.59761, -0.71714, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);
    assert_eq!(t, expected_transform)
}

#[test]
fn camera_creation() {
    let c = camera(160, 120, PI / 2.);
    assert_eq!(c.hsize, 160);
    assert_eq!(c.vsize, 120);
    assert_eq!(c.field_of_view, PI / 2.);
    assert_eq!(c.transform, identity());
}

#[test]
fn pixel_size_for_horizontal_canvas() {
    let c = camera(200, 125, PI / 2.);
    assert_eq!(c.pixel_size, 0.01);
}

#[test]
fn pixel_size_for_vertical_canvas() {
    let c = camera(125, 200, PI / 2.);
    assert_eq!(c.pixel_size, 0.01);
}

#[test]
fn constructing_ray_through_center_of_canvas() {
    let c = camera(201, 101, PI / 2.);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, point(0., 0., 0.));
    assert_eq!(r.direction, vector(0., 0., -1.));
}

#[test]
fn constructing_ray_through_corner_of_canvas() {
    let c = camera(201, 101, PI / 2.);
    let r = c.ray_for_pixel(0, 0);
    assert_eq!(r.origin, point(0., 0., 0.));
    assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn constructing_ray_when_camera_is_transformed() {
    let mut c = camera(201, 101, PI / 2.);
    c.transform = rotation_y(PI / 4.) * translation(0., -2., 5.);
    let sqrt_of_2_over_2 = (2.0 as f32).sqrt() / 2.0;
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, point(0., 2., -5.));

    assert_eq!(
        r.direction,
        vector(sqrt_of_2_over_2, 0.0, -sqrt_of_2_over_2)
    );
}

#[test]
fn rendering_a_world_with_a_camera() {
    let w = default_world();
    let mut c = camera(11, 11, PI / 2.);

    let from = point(0., 0., -5.);
    let to = point(0., 0., 0.);
    let up = vector(0., 1., 0.);
    c.transform = view_transform(from, to, up);

    let image = render(c, w);

    assert_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855))
}
