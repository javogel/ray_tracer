use ray_tracer::{
    color::color,
    pattern::*,
    shapes::object::Object,
    transforms::{scaling, translation},
    tuple::point,
};

#[test]
fn creating_a_stripe_pattern() {
    let a = color(1., 1., 1.);
    let b = color(0., 0., 0.);
    let pattern = stripe_pattern(a, b);
    assert_eq!(pattern.a, a);
    assert_eq!(pattern.b, b);
}

#[test]
fn stripe_pattern_is_constant_in_y() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = stripe_pattern(white, black);
    assert_eq!(pattern.at_point(point(0., 0., 0.)), white);
    assert_eq!(pattern.at_point(point(0., 1., 0.)), white);
    assert_eq!(pattern.at_point(point(0., 2., 0.)), white);
}

#[test]
fn stripe_pattern_is_constant_in_z() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = stripe_pattern(white, black);
    assert_eq!(pattern.at_point(point(0., 0., 0.)), white);
    assert_eq!(pattern.at_point(point(0., 0., 1.)), white);
    assert_eq!(pattern.at_point(point(0., 0., 2.)), white);
}

#[test]
fn stripe_pattern_alternates_in_x() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = stripe_pattern(white, black);
    assert_eq!(pattern.at_point(point(0., 0., 0.)), white);
    assert_eq!(pattern.at_point(point(0.9, 0., 0.)), white);
    assert_eq!(pattern.at_point(point(1., 0., 0.)), black);
    assert_eq!(pattern.at_point(point(-0.1, 0., 0.)), black);
    assert_eq!(pattern.at_point(point(-1., 0., 0.)), black);
    assert_eq!(pattern.at_point(point(-1.1, 0., 0.)), white);
}

#[test]
fn stripes_with_an_object_transformation() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = stripe_pattern(white, black);
    let mut object = Object::new_sphere();
    object.set_transform(scaling(2., 2., 2.));
    assert_eq!(pattern.at_object(&object, point(1.5, 0., 0.)), white);
}

#[test]
fn stripes_with_a_pattern_transformation() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let mut pattern = stripe_pattern(white, black);
    let object = Object::new_sphere();
    pattern.transform = scaling(2., 2., 2.);
    assert_eq!(pattern.at_object(&object, point(1.5, 0., 0.)), white);
}

#[test]
fn stripes_with_a_pattern_and_object_transformation() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let mut pattern = stripe_pattern(white, black);
    pattern.transform = scaling(2., 2., 2.);

    let mut object = Object::new_sphere();
    object.set_transform(translation(0.5, 0., 0.));

    assert_eq!(pattern.at_object(&object, point(1.5, 0., 0.)), white);
}

#[test]
fn gradient_linearly_interpoilates_between_colors() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = gradient_pattern(white, black);
    assert_eq!(pattern.at_point(point(0., 0., 0.)), white);
    assert_eq!(
        pattern.at_point(point(0.25, 0., 0.)),
        color(0.75, 0.75, 0.75)
    );
    assert_eq!(pattern.at_point(point(0.5, 0., 0.)), color(0.5, 0.5, 0.5));
    assert_eq!(
        pattern.at_point(point(0.75, 0., 0.)),
        color(0.25, 0.25, 0.25)
    );
}

#[test]
fn ring_pattern_should_extend_in_both_x_and_z() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = ring_pattern(white, black);
    assert_eq!(pattern.at_point(point(0., 0., 0.)), white);
    assert_eq!(pattern.at_point(point(1., 0., 0.)), black);
    assert_eq!(pattern.at_point(point(0., 0., 1.)), black);
    assert_eq!(pattern.at_point(point(0.708, 0., 708.)), black);
}

#[test]
fn checkers_should_repeat_in_x() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = checker_pattern(white, black);
    assert_eq!(pattern.at_point(point(0., 0., 0.)), white);
    assert_eq!(pattern.at_point(point(0.99, 0., 0.)), white);
    assert_eq!(pattern.at_point(point(1.01, 0., 0.)), black);
}

#[test]
fn checkers_should_repeat_in_y() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = checker_pattern(white, black);
    assert_eq!(pattern.at_point(point(0., 0., 0.)), white);
    assert_eq!(pattern.at_point(point(0., 0.99, 0.)), white);
    assert_eq!(pattern.at_point(point(0., 1.01, 0.)), black);
}

#[test]
fn checkers_should_repeat_in_z() {
    let white = color(1., 1., 1.);
    let black = color(0., 0., 0.);
    let pattern = checker_pattern(white, black);
    assert_eq!(pattern.at_point(point(0., 0., 0.)), white);
    assert_eq!(pattern.at_point(point(0., 0., 0.99)), white);
    assert_eq!(pattern.at_point(point(0., 0., 1.01)), black);
}
