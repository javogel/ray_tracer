use ray_tracer::{color::color, material::*};

#[test]
fn creating_default_material() {
    let material = material();

    assert_eq!(material.color, color(1., 1., 1.));
    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.9);
    assert_eq!(material.specular, 0.9);
    assert_eq!(material.shininess, 200.);
    assert_eq!(material.reflective, 0.);
}
