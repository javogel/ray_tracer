use ray_tracer::color::*;

#[test]
fn test_addition() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
}

#[test]
fn test_subtraction() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
}

#[test]
fn test_scalar_multiplication() {
    let c1 = color(0.2, 0.3, 0.4);
    assert_eq!(c1 * 2.0, color(0.4, 0.6, 0.8));
    assert_eq!(multiply(c1, 2.0), color(0.4, 0.6, 0.8));
}

#[test]
fn test_color_multiplication() {
    let c1 = color(1.0, 0.2, 0.4);
    let c2 = color(0.9, 1., 0.1);
    assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    assert_eq!(multiply(c1, c2), color(0.9, 0.2, 0.04));
}
