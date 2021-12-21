use ray_tracer::{
    material::material,
    matrix::identity,
    shapes::object::Object,
    transforms::{rotation_z, scaling, translation},
    tuple::*,
};
use std::f64::consts::PI;

#[test]
fn normal_on_sphere_at_point_on_x_axis() {
    let s = Object::new_sphere();
    let normal = s.normal_at(point(1., 0., 0.));

    assert_eq!(normal, vector(1., 0., 0.));
}

#[test]
fn normal_on_sphere_at_point_on_y_axis() {
    let s = Object::new_sphere();
    let normal = s.normal_at(point(0., 1., 0.));

    assert_eq!(normal, vector(0., 1., 0.));
}

#[test]
fn normal_on_sphere_at_point_on_z_axis() {
    let s = Object::new_sphere();
    let normal = s.normal_at(point(0., 0., 1.));

    assert_eq!(normal, vector(0., 0., 1.));
}

#[test]
fn normal_on_sphere_at_point_on_nonaxial_point() {
    let sqrt_of_3_over_3 = (3.0 as f64).sqrt() / 3.0;
    let s = Object::new_sphere();
    let normal = s.normal_at(point(sqrt_of_3_over_3, sqrt_of_3_over_3, sqrt_of_3_over_3));

    assert_eq!(
        normal,
        vector(sqrt_of_3_over_3, sqrt_of_3_over_3, sqrt_of_3_over_3)
    );
}

#[test]
fn normal_on_sphere_is_normalized() {
    let sqrt_of_3_over_3 = (3.0 as f64).sqrt() / 3.0;
    let s = Object::new_sphere();
    let normal = s.normal_at(point(sqrt_of_3_over_3, sqrt_of_3_over_3, sqrt_of_3_over_3));

    assert_eq!(
        normal,
        vector(sqrt_of_3_over_3, sqrt_of_3_over_3, sqrt_of_3_over_3).normalize()
    );
}

#[test]
fn computing_normal_on_translated_sphere() {
    let mut s = Object::new_sphere();
    s.set_transform(translation(0., 1., 0.));

    let normal = s.normal_at(point(0., 1.70711, -0.70711));
    assert_eq!(normal, vector(0., 0.70711, -0.70711));
}

#[test]
fn computing_normal_on_transformed_sphere() {
    let sqrt_of_2_over_2 = (2.0 as f64).sqrt() / 2.0;
    let mut s = Object::new_sphere();
    s.set_transform(scaling(1., 0.5, 1.) * rotation_z(PI / 5.0));

    let normal = s.normal_at(point(0., sqrt_of_2_over_2, -sqrt_of_2_over_2));
    assert_eq!(normal, vector(0., 0.97014, -0.24254));
}

#[test]
fn sphere_has_default_material() {
    let s = Object::new_sphere();
    assert!(s.material == material());
}

#[test]
fn sphere_can_be_assigned_material() {
    let mut s = Object::new_sphere();
    let mut m = material();
    m.ambient = 1.0;
    s.material = m.clone();
    assert!(s.material == m);
}

#[test]
fn sphere_default_transform() {
    let s = Object::new_sphere();

    assert_eq!(s.transform, identity());
}

#[test]
fn change_to_sphere_transform() {
    let mut s = Object::new_sphere();
    let t = translation(2., 3., 4.);
    s.set_transform(t.clone());
    assert_eq!(s.transform, t);
}

#[test]
fn change_to_sphere_transform() {
    let mut s = Object::new_glass_sphere();
    let m = &s.material;
    assert_eq!(s.transparency, 1.);
    assert_eq!(s.refractive_index, 1.5);
}
