use ray_tracer::ray::*;
use ray_tracer::shapes::object::*;
use ray_tracer::transforms::scaling;
use ray_tracer::transforms::translation;
use ray_tracer::tuple::*;
use ray_tracer::utils::EPSILON;
use ray_tracer::world::prepare_computations;

#[test]
fn ray_creation() {
    let origin = point(1., 2., 3.);
    let direction = vector(4., 5., 6.);
    let r = ray(origin, direction);

    assert_eq!(r.origin, origin);
    assert_eq!(r.direction, direction);
}

#[test]
fn ray_position() {
    let origin = point(2., 3., 4.);
    let direction = vector(1., 0., 0.);
    let r = ray(origin, direction);

    assert_eq!(r.position(0.), point(2., 3., 4.));
    assert_eq!(r.position(1.), point(3., 3., 4.));
    assert_eq!(r.position(-1.), point(1., 3., 4.));
    assert_eq!(r.position(2.5), point(4.5, 3., 4.));
}

#[test]
fn ray_intersect_at_tangent() {
    let origin = point(0., 1., -5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);

    let shape = Object::new_sphere();
    let xs = intersect(&r, &shape);
    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, 5.);
    assert_eq!(xs[1].t, 5.);
}

#[test]
fn ray_intersect_misses_sphere() {
    let origin = point(0., 2., -5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let shape = Object::new_sphere();
    let xs = intersect(&r, &shape);
    assert_eq!(xs.count(), 0);
}

#[test]
fn ray_intersect_inside_sphere() {
    let origin = point(0., 0., 0.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let shape = Object::new_sphere();
    let xs = intersect(&r, &shape);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, -1.);
    assert_eq!(xs[1].t, 1.);
}

#[test]
fn intersect_sphere_behind_ray() {
    let origin = point(0., 0., 5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let shape = Object::new_sphere();
    let xs = intersect(&r, &shape);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, -6.);
    assert_eq!(xs[1].t, -4.);
}

#[test]
fn intersect_contains_t_and_uuid() {
    let shape = Object::new_sphere();
    let i = intersection(3.5, &shape);

    assert_eq!(i.t, 3.5);
    assert_eq!(i.object.uuid, shape.uuid);
}

#[test]
fn aggregating_intersections() {
    let shape = Object::new_sphere();
    let i1 = intersection(1., &shape);
    let i2 = intersection(2., &shape);

    let xs = intersections(vec![i1, i2]);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].object.uuid, shape.uuid);
    assert_eq!(xs[1].object.uuid, shape.uuid);
}

#[test]
fn intersect_sets_object_uuid() {
    let origin = point(0., 0., -5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);
    let shape = Object::new_sphere();
    let xs = intersect(&r, &shape);

    assert_eq!(xs.count(), 2);

    assert_eq!(xs[0].object.uuid, shape.uuid);
    assert_eq!(xs[1].object.uuid, shape.uuid);
}

#[test]
fn hit_when_intersections_have_positive_t() {
    let shape = Object::new_sphere();
    let i1 = intersection(1., &shape);
    let i2 = intersection(2., &shape);

    let xs = intersections(vec![i2, i1.clone()]);

    assert!(xs.hit().unwrap() == i1);
}

#[test]
fn hit_when_intersections_have_negative_t() {
    let shape = Object::new_sphere();
    let i1 = intersection(-1., &shape);
    let i2 = intersection(1., &shape);

    let xs = intersections(vec![i2.clone(), i1]);

    assert!(xs.hit().unwrap() == i2);
}

#[test]
fn hit_when_all_intersections_have_negative_t() {
    let shape = Object::new_sphere();
    let i1 = intersection(-2., &shape);
    let i2 = intersection(-1., &shape);

    let xs = intersections(vec![i2, i1]);

    assert!(xs.hit() == None);
}

#[test]
fn hit_as_lowest_nonnegative_t() {
    let shape = &Object::new_sphere();
    let i1 = intersection(5., shape);
    let i2 = intersection(7., shape);
    let i3 = intersection(-3., shape);
    let i4 = intersection(2., shape);

    let xs = intersections(vec![i1, i2, i3, i4.clone()]);

    assert!(xs.hit() == Some(i4));
}

#[test]
fn translating_a_ray() {
    let origin = point(1., 2., 3.);
    let direction = vector(0., 1., 0.);
    let r = ray(origin, direction);

    let translation = translation(3., 4., 5.);
    let Ray { origin, direction } = r.transform(&translation);

    assert_eq!(origin, point(4., 6., 8.));
    assert_eq!(direction, vector(0., 1., 0.));
}

#[test]
fn scaling_a_ray() {
    let origin = point(1., 2., 3.);
    let direction = vector(0., 1., 0.);
    let r = ray(origin, direction);

    let transform = scaling(2., 3., 4.);
    let Ray { origin, direction } = r.transform(&transform);

    assert_eq!(origin, point(2., 6., 12.));
    assert_eq!(direction, vector(0., 3., 0.));
}

#[test]
fn intersect_scaled_sphere_with_ray() {
    let origin = point(0., 0., -5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);

    let mut shape_object = Object::new_sphere();

    shape_object.set_transform(scaling(2., 2., 2.));

    let xs = intersect(&r, &shape_object);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, 3.);
    assert_eq!(xs[1].t, 7.);
}

#[test]
fn intersect_translated_sphere_with_ray() {
    let origin = point(0., 0., -5.);
    let direction = vector(0., 0., 1.);
    let r = ray(origin, direction);

    let mut shape = Object::new_sphere();
    shape.set_transform(translation(5., 0., 0.));
    let xs = intersect(&r, &shape);
    assert_eq!(xs.count(), 0);
}

#[test]
fn finding_n1_and_n2_at_various_intersections() {
    let mut a = Object::new_glass_sphere();
    a.transform = scaling(2., 2., 2.);
    a.material.refractive_index = 1.5;

    let mut b = Object::new_glass_sphere();
    b.transform = translation(0., 0., -0.25);
    b.material.refractive_index = 2.0;

    let mut c = Object::new_glass_sphere();
    c.transform = translation(0., 0., 0.25);
    c.material.refractive_index = 2.5;

    let r = ray(point(0., 0., -4.), vector(0., 0., 1.));
    let xs = intersections(vec![
        intersection(2.0, &a),
        intersection(2.75, &b),
        intersection(3.25, &c),
        intersection(4.75, &b),
        intersection(5.25, &c),
        intersection(6., &a),
    ]);

    let expected_n = [
        [1.0, 1.5],
        [1.5, 2.0],
        [2.0, 2.5],
        [2.5, 2.5],
        [2.5, 1.5],
        [1.5, 1.0],
    ];
    for (index, element) in xs.locations.iter().enumerate() {
        let comps = prepare_computations(element, &r, &xs);
        assert_eq!(expected_n[index][0], comps.n1);
        assert_eq!(expected_n[index][1], comps.n2);
    }
}

#[test]
fn the_under_point_is_offset_below_the_surface() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let mut shape = Object::new_glass_sphere();
    shape.transform = translation(0., 0., 1.);
    let xs = intersections(vec![intersection(5.0, &shape)]);
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    assert!(comps.under_point.z > EPSILON / 2.);
    assert!(comps.point.z < comps.under_point.z);
}
