use ray_tracer::transforms::*;
use ray_tracer::tuple::*;
use std::f64::consts::PI;

#[test]
fn point_translation() {
    let transform = translation(5., -3., 2.);

    let p = point(-3., 4., 5.);
    assert_eq!(transform.clone() * p.clone(), point(2., 1., 7.));

    let inverse_transform = transform.inverse().unwrap();
    assert_eq!(inverse_transform * p, point(-8., 7., 3.));
}

#[test]
fn vector_translation() {
    let transform = translation(5., -3., 2.);

    let v = vector(-3., 4., 5.);

    assert_eq!(transform * v.clone(), v);
}

#[test]
fn point_scaling() {
    let transform = scaling(2., 3., 4.);

    let p = point(-4., 6., 8.);
    assert_eq!(transform * p, point(-8., 18., 32.));
}

#[test]
fn vector_scaling() {
    let transform = scaling(2., 3., 4.);

    let v = vector(-4., 6., 8.);
    assert_eq!(transform.clone() * v, vector(-8., 18., 32.));

    let inverse_transform = transform.inverse().unwrap();
    assert_eq!(inverse_transform * v, vector(-2., 2., 2.));
}

#[test]
fn reflection_scaling() {
    let transform = scaling(-1., 1., 1.);

    let p = point(2., 3., 4.);
    assert_eq!(transform.clone() * p, point(-2., 3., 4.));
}

#[test]
fn rotation_x_axis() {
    let half_quarter = rotation_x(PI / 4.);
    let full_quarter = rotation_x(PI / 2.);

    let p = point(0., 1., 0.);

    let square_root_of_2 = (2. as f64).sqrt();
    assert_eq!(
        half_quarter.clone() * p,
        point(0., square_root_of_2 / 2., square_root_of_2 / 2.)
    );

    assert_eq!(
        half_quarter.inverse().unwrap() * p,
        point(0., square_root_of_2 / 2., -square_root_of_2 / 2.)
    );

    assert_eq!(full_quarter * p, point(0., 0., 1.));
}

#[test]
fn rotation_inverse() {
    let half_quarter = rotation_x(PI / 4.);
    let p = point(0., 1., 0.);
    let square_root_of_2 = (2. as f64).sqrt();

    assert_eq!(
        half_quarter.inverse().unwrap() * p,
        point(0., square_root_of_2 / 2., -square_root_of_2 / 2.)
    );
}

#[test]
fn rotation_y_axis() {
    let half_quarter = rotation_y(PI / 4.);
    let full_quarter = rotation_y(PI / 2.);

    let p = point(0., 0., 1.);

    let square_root_of_2 = (2. as f64).sqrt();
    assert_eq!(
        half_quarter.clone() * p,
        point(square_root_of_2 / 2., 0., square_root_of_2 / 2.)
    );

    assert_eq!(full_quarter * p, point(1., 0., 0.));
}

#[test]
fn rotation_z_axis() {
    let half_quarter = rotation_z(PI / 4.);
    let full_quarter = rotation_z(PI / 2.);

    let p = point(0., 1., 0.);

    let square_root_of_2 = (2. as f64).sqrt();
    assert_eq!(
        half_quarter.clone() * p,
        point(-square_root_of_2 / 2., square_root_of_2 / 2., 0.)
    );

    assert_eq!(full_quarter * p, point(-1., 0., 0.));
}

#[test]
fn shearing_transform() {
    let p = point(2.0, 3.0, 4.0);

    let mut transform = shearing(1.0, 0., 0., 0., 0., 0.);
    assert_eq!(transform * p, point(5., 3., 4.));

    transform = shearing(0.0, 1., 0., 0., 0., 0.);
    assert_eq!(transform * p, point(6., 3., 4.));

    transform = shearing(0.0, 0., 1., 0., 0., 0.);
    assert_eq!(transform * p, point(2., 5., 4.));

    transform = shearing(0.0, 0., 0., 1., 0., 0.);
    assert_eq!(transform * p, point(2., 7., 4.));

    transform = shearing(0.0, 0., 0., 0., 1., 0.);
    assert_eq!(transform * p, point(2., 3., 6.));

    transform = shearing(0.0, 0., 0., 0., 0., 1.);
    assert_eq!(transform * p, point(2., 3., 7.));
}

#[test]
fn sequence_of_transformations() {
    let p = point(1.0, 0., 1.);
    let a = rotation_x(PI / 2.);
    let b = scaling(5., 5., 5.);
    let c = translation(10., 5.0, 7.);

    let p2 = a * p;
    assert_eq!(p2, point(1., -1., 0.));

    let p3 = b * p2;
    assert_eq!(p3, point(5., -5., 0.));

    let p4 = c * p3;
    assert_eq!(p4, point(15., 0., 7.));
}

#[test]
fn chained_transformations() {
    let p = point(1.0, 0., 1.);
    let a = rotation_x(PI / 2.);
    let b = scaling(5., 5., 5.);
    let c = translation(10., 5.0, 7.);

    assert_eq!((c * b * a) * p, point(15., 0., 7.));
}

#[test]
fn chained_transformations_api() {
    let p = point(1.0, 0., 1.)
        .rotate_x(PI / 2.)
        .scale(5., 5., 5.)
        .translate(10., 5.0, 7.);

    assert_eq!(p, point(15., 0., 7.));
}
