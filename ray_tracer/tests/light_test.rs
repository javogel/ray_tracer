use ray_tracer::{
    color::color,
    light::{is_shadowed, lighting, point_light},
    material::material,
    shapes::object::Object,
    tuple::{point, vector},
    world::default_world,
};

#[test]
fn point_light_has_intensity_and_position() {
    let intensity = color(1., 1., 1.);
    let position = point(0., 0., 0.);

    let light = point_light(position, intensity);
    assert_eq!(light.intensity, intensity);
    assert_eq!(light.position, position);
}

#[test]
fn lighting_with_eye_between_light_and_surface() {
    let m = material();
    let position = point(0., 0., 0.);
    let eyev = vector(0., 0., -1.);
    let normalv = vector(0., 0., -1.);
    let light = point_light(point(0., 0., -10.), color(1., 1., 1.));
    let result = lighting(
        &m,
        &Object::new_sphere(),
        &light,
        position,
        eyev,
        normalv,
        false,
    );

    assert_eq!(result, color(1.9, 1.9, 1.9));
}

#[test]
fn lighting_with_eye_between_light_and_surface_and_eye_offset_45() {
    let sqrt_of_2_over_2 = (2.0 as f64).sqrt() / 2.0;
    let m = material();
    let position = point(0., 0., 0.);
    let eyev = vector(0., sqrt_of_2_over_2, sqrt_of_2_over_2);
    let normalv = vector(0., 0., -1.);
    let light = point_light(point(0., 0., -10.), color(1., 1., 1.));
    let result = lighting(
        &m,
        &Object::new_sphere(),
        &light,
        position,
        eyev,
        normalv,
        false,
    );

    assert_eq!(result, color(1., 1., 1.));
}

#[test]
fn lighting_with_eye_opposite_surface_and_light_offset_45() {
    let m = material();
    let position = point(0., 0., 0.);
    let eyev = vector(0., 0., -1.);
    let normalv = vector(0., 0., -1.);
    let light = point_light(point(0., 10., -10.), color(1., 1., 1.));
    let result = lighting(
        &m,
        &Object::new_sphere(),
        &light,
        position,
        eyev,
        normalv,
        false,
    );

    assert_eq!(result, color(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_with_eye_in_path_of_reflection_vector() {
    let sqrt_of_2_over_2 = (2.0 as f64).sqrt() / 2.0;
    let m = material();
    let position = point(0., 0., 0.);
    let eyev = vector(0., -sqrt_of_2_over_2, -sqrt_of_2_over_2);
    let normalv = vector(0., 0., -1.);
    let light = point_light(point(0., 10., -10.), color(1., 1., 1.));
    let result = lighting(
        &m,
        &Object::new_sphere(),
        &light,
        position,
        eyev,
        normalv,
        false,
    );

    assert_eq!(result, color(1.6364, 1.6364, 1.6364));
}

#[test]
fn lighting_with_light_behind_surface() {
    let m = material();
    let position = point(0., 0., 0.);
    let eyev = vector(0., 0., -1.);
    let normalv = vector(0., 0., -1.);
    let light = point_light(point(0., 0., 10.), color(1., 1., 1.));
    let result = lighting(
        &m,
        &Object::new_sphere(),
        &light,
        position,
        eyev,
        normalv,
        false,
    );

    assert_eq!(result, color(0.1, 0.1, 0.1));
}

#[test]
fn lighting_with_surface_in_shadow() {
    let m = material();
    let position = point(0., 0., 0.);
    let eyev = vector(0., 0., -1.);
    let normalv = vector(0., 0., -1.);
    let light = point_light(point(0., 0., -10.), color(1., 1., 1.));
    let in_shadow = true;
    let result = lighting(
        &m,
        &Object::new_sphere(),
        &light,
        position,
        eyev,
        normalv,
        in_shadow,
    );

    assert_eq!(result, color(0.1, 0.1, 0.1));
}

#[test]
fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
    let w = default_world();
    let p = point(0., 10., 0.);

    assert_eq!(is_shadowed(&w, p), false);
}

#[test]
fn the_shadow_when_object_is_between_point_and_light() {
    let w = default_world();
    let p = point(10., -10., 10.);

    assert_eq!(is_shadowed(&w, p), true);
}

#[test]
fn the_is_no_shadow_when_object_is_behind_light() {
    let w = default_world();
    let p = point(-20., 20., -20.);

    assert_eq!(is_shadowed(&w, p), false);
}

#[test]
fn the_is_no_shadow_when_object_is_behind_point() {
    let w = default_world();
    let p = point(-2., 2., -2.);

    assert_eq!(is_shadowed(&w, p), false);
}
