use ray_tracer::tuple::*;

#[test]
fn test_is_a_vector() {
    let point = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: TupleType::Point,
    };
    assert_eq!(is_a_vector(point), false);

    let vector = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: TupleType::Vector,
    };
    assert_eq!(is_a_vector(vector), true);
}

#[test]
fn test_point() {
    let Tuple { x, y, z, w } = point(4., -4., 3.);
    assert_eq!((x, y, z), (4., -4., 3.));
    assert_eq!(w, TupleType::Point);
}

#[test]
fn test_vector() {
    let Tuple { x, y, z, w } = vector(4., -4., 3.);
    assert_eq!((x, y, z), (4., -4., 3.));
    assert_eq!(w, TupleType::Vector);
}

#[test]
fn test_equal() {
    assert_eq!(equal(1.0, 1.0), true);
    assert_eq!(equal(1.0, 2.0), false);
}

#[test]
fn test_add() {
    let a = vector(3., -2., 5.);
    let b = point(-2., 3., 1.);
    let result = point(1., 1., 6.);
    assert_eq!(add(a.clone(), b.clone()), result);
    assert_eq!(add(b, a), result);
}

#[test]
fn test_subtract() {
    let a = point(3., 2., 1.);
    let mut b = point(5., 6., 7.);
    assert_eq!(subtract(a, b), vector(-2., -4., -6.));

    let c = point(3., 2., 1.);
    b = vector(5., 6., 7.);
    assert_eq!(subtract(c, b), point(-2., -4., -6.));

    let e = point(3., 2., 1.);
    b = point(5., 6., 7.);
    assert_eq!(subtract(e, b), vector(-2., -4., -6.));
}

#[test]
fn test_negate() {
    let a = point(3., 2., 1.);
    assert_eq!(negate(a), point(-3., -2., -1.));
}

#[test]
fn test_multiply() {
    let a = point(1., -2., 3.);
    let mut s = 2.0;
    assert_eq!(multiply(a, s), point(2., -4., 6.));

    s = 0.5;
    assert_eq!(multiply(a, s), point(0.5, -1., 1.5));
}

#[test]
fn test_divide() {
    let a = point(1., -2., 3.);
    let s = 2.0;
    assert_eq!(divide(a, s), point(0.5, -1., 1.5));
}

#[test]
fn test_magnitude() {
    let mut m = magnitude(vector(1., 0., 0.));
    assert_eq!(m, 1.0);

    m = magnitude(vector(0., 1., 0.));
    assert_eq!(m, 1.0);

    m = magnitude(vector(0., 0., 1.));
    assert_eq!(m, 1.0);

    m = magnitude(vector(1., 2., 3.));
    assert_eq!(m, 14.0_f32.sqrt());

    m = magnitude(vector(-1., -2., -3.));
    assert_eq!(m, 14.0_f32.sqrt());
}

#[test]
fn test_normalize() {
    let mut n = normalize(vector(4., 0., 0.));
    assert_eq!(n, vector(1., 0., 0.));

    let scalar = 1.0 / 14.0_f32.sqrt();
    n = normalize(vector(1., 2., 3.));
    assert_eq!(n, vector(scalar, 2.0 * scalar, 3.0 * scalar,));
    assert!(equal(magnitude(n), 1.0));
}

#[test]
fn test_dot() {
    let a = vector(1., 2., 3.);
    let b = vector(2., 3., 4.);
    let d = dot(a, b);
    assert_eq!(d, 20.);
}

#[test]
fn test_cross() {
    let a = vector(1., 2., 3.);
    let b = vector(2., 3., 4.);
    assert_eq!(cross(a, b), vector(-1., 2., -1.));
    assert_eq!(cross(b, a), vector(1., -2., 1.));
}
