use ray_tracer::{
    ray::*,
    shapes::{object::Object, plane::plane},
    tuple::*,
};

#[test]
fn normal_on_plane_is_constant_everywhere() {
    let p = plane();

    assert_eq!(p.local_normal_at(point(0., 0., 0.)), vector(0., 1., 0.));
    assert_eq!(p.local_normal_at(point(10., 0., -10.)), vector(0., 1., 0.));
    assert_eq!(p.local_normal_at(point(-5., 0., 150.)), vector(0., 1., 0.));
}

#[test]
fn intersect_with_ray_parallel_to_plane() {
    let p = Object::new_plane();
    let r = ray(point(0., 10., 0.), vector(0., 0., 1.));

    let intersect = p.intersect(&r);
    assert_eq!(intersect.locations.len(), 0);
}

#[test]
fn intersect_with_coplanar_ray() {
    let p = Object::new_plane();
    let r = ray(point(0., 0., 0.), vector(0., 0., 1.));

    let intersect = p.intersect(&r);
    assert_eq!(intersect.locations.len(), 0);
}

#[test]
fn ray_intersecting_plane_from_above() {
    let p = Object::new_plane();
    let r = ray(point(0., 1., 0.), vector(0., -1., 0.));

    let intersect = p.intersect(&r);
    assert_eq!(intersect.locations.len(), 1);
    assert_eq!(intersect.locations[0].t, 1.);
    assert!(intersect.locations[0].object == &p);
}

#[test]
fn ray_intersecting_plane_from_below() {
    let p = Object::new_plane();
    let r = ray(point(0., -1., 0.), vector(0., 1., 0.));

    let intersect = p.intersect(&r);
    assert_eq!(intersect.locations.len(), 1);
    assert_eq!(intersect.locations[0].t, 1.);
    assert!(intersect.locations[0].object == &p);
}
