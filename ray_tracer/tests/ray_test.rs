use ray_tracer::ray::*;
use ray_tracer::tuple::*;

#[test]
fn test_ray_creation() {
    let origin = point(1., 2., 3.);
    let direction = vector(4., 5., 6.);
    let r = ray(origin, direction);

    assert_eq!(r.origin, origin);
    assert_eq!(r.direction, direction);
}

#[test]
fn test_ray_position() {
    let origin = point(2., 3., 4.);
    let direction = vector(1., 0., 0.);
    let r = ray(origin, direction);

    assert_eq!(r.position(0.), point(2., 3., 4.));
    assert_eq!(r.position(1.), point(3., 3., 4.));
    assert_eq!(r.position(-1.), point(1., 3., 4.));
    assert_eq!(r.position(2.5), point(4.5, 3., 4.));
}

#[test]
fn test_ray_intersect_at_tangent() {
    let origin = point(0., 1., -5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let s = sphere(point(0., 0., 0.), 1.);

    let xs = intersect(r, s);
    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, 5.);
    assert_eq!(xs[1].t, 5.);
}

#[test]
fn test_ray_intersect_misses_sphere() {
    let origin = point(0., 2., -5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let s = sphere(point(0., 0., 0.), 1.);

    let xs = intersect(r, s);
    assert_eq!(xs.count(), 0);
}

#[test]
fn test_ray_intersect_inside_sphere() {
    let origin = point(0., 0., 0.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let s = sphere(point(0., 0., 0.), 2.);

    let xs = intersect(r, s);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, -1.);
    assert_eq!(xs[1].t, 1.);
}

#[test]
fn test_intersect_sphere_behind_ray() {
    let origin = point(0., 0., 5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let s = sphere(point(0., 0., 0.), 1.);

    let xs = intersect(r, s);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, -6.);
    assert_eq!(xs[1].t, -4.);
}

#[test]
fn test_intersect_contains_t_and_uuid() {
    let s = sphere(point(0., 0., 0.), 1.);
    let i = intersection(3.5, s.uuid);

    assert_eq!(i.t, 3.5);
    assert_eq!(i.object_uuid, s.uuid);
}

#[test]
fn test_aggregating_intersections() {
    let s = sphere(point(0., 0., 0.), 1.);
    let i1 = intersection(1., s.uuid);
    let i2 = intersection(2., s.uuid);

    let xs = intersections(vec![i1, i2]);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].object_uuid, s.uuid);
    assert_eq!(xs[1].object_uuid, s.uuid);
}

#[test]
fn test_intersect_sets_object_uuid() {
    let origin = point(0., 0., -5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let s = sphere(point(0., 0., 0.), 1.);

    let xs = intersect(r, s.clone());

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].object_uuid, s.uuid);
    assert_eq!(xs[1].object_uuid, s.uuid);
}

#[test]
fn test_hit_when_intersections_have_positive_t() {
    let s = sphere(point(0., 0., 0.), 1.);
    let i1 = intersection(1., s.uuid);
    let i2 = intersection(2., s.uuid);

    let xs = intersections(vec![i2, i1.clone()]);

    assert_eq!(xs.hit().unwrap(), i1);
}

#[test]
fn test_hit_when_intersections_have_negative_t() {
    let s = sphere(point(0., 0., 0.), 1.);
    let i1 = intersection(-1., s.uuid);
    let i2 = intersection(1., s.uuid);

    let xs = intersections(vec![i2.clone(), i1]);

    assert_eq!(xs.hit().unwrap(), i2);
}

#[test]
fn test_hit_when_all_intersections_have_negative_t() {
    let s = sphere(point(0., 0., 0.), 1.);
    let i1 = intersection(-2., s.uuid);
    let i2 = intersection(-1., s.uuid);

    let xs = intersections(vec![i2, i1]);

    assert_eq!(xs.hit(), None);
}

#[test]
fn test_hit_as_lowest_nonnegative_t() {
    let s = sphere(point(0., 0., 0.), 1.);
    let i1 = intersection(5., s.uuid);
    let i2 = intersection(7., s.uuid);
    let i3 = intersection(-3., s.uuid);
    let i4 = intersection(2., s.uuid);

    let xs = intersections(vec![i1, i2, i3, i4.clone()]);

    assert_eq!(xs.hit(), Some(i4));
}
