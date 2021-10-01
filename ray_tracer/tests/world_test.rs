use ray_tracer::{
    color::color,
    light::point_light,
    material::Material,
    ray::*,
    shapes::object::*,
    transforms::{scaling, translation},
    tuple::{point, vector},
    utils::EPSILON,
    world::{default_world, *},
};

#[test]
fn world_can_have_light_and_no_objects() {
    let origin = point(-10., 10., -10.);
    let light_color = color(1., 1., 1.);
    let light = point_light(origin, light_color);

    let obj = Object::new_sphere();

    let w = world(light, vec![obj.clone()]);

    assert_eq!(w.light.position, origin);
    assert_eq!(w.light.intensity, light_color);
    assert_eq!(w.objects[0], obj);
}

#[test]
fn default_world_has_lighting_and_2_spheres() {
    let world = default_world();

    assert_eq!(world.light.position, point(-10., 10., -10.));
    assert_eq!(world.light.intensity, color(1., 1., 1.));

    assert_eq!(world.objects[0].material().color, color(0.8, 1., 0.6));
    assert_eq!(world.objects[0].material().diffuse, 0.7);
    assert_eq!(world.objects[0].material().specular, 0.2);

    assert_eq!(*world.objects[1].transform(), scaling(0.5, 0.5, 0.5));
}

#[test]
fn intersect_world_with_ray() {
    let w = default_world();
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let result = w.intersect(&r);

    assert_eq!(result.locations[0].t, 4.);
    assert_eq!(result.locations[1].t, 4.5);
    assert_eq!(result.locations[2].t, 5.5);
    assert_eq!(result.locations[3].t, 6.);
}

#[test]
fn pre_computating_state_of_intersection() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let shape = Object::new_sphere();
    let i = intersection(4., &shape);

    let comps = prepare_computations(&i, &r);
    assert_eq!(comps.t, 4.);
    assert_eq!(comps.object.uuid(), shape.uuid());
    assert_eq!(comps.point, point(0., 0., -1.));
    assert_eq!(comps.eyev, vector(0., 0., -1.));
    assert_eq!(comps.normalv, vector(0., 0., -1.));
}

#[test]
fn pre_computing_state_of_intersection_when_outside() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let shape = Object::new_sphere();
    let i = intersection(4., &shape);

    let comps = prepare_computations(&i, &r);
    assert_eq!(comps.inside, false);
}

#[test]
fn pre_computing_state_of_intersection_when_inside() {
    let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
    let shape = Object::new_sphere();
    let i = intersection(1., &shape);

    let comps = prepare_computations(&i, &r);
    assert_eq!(comps.inside, true);
    assert_eq!(comps.point, point(0., 0., 1.));
    assert_eq!(comps.eyev, vector(0., 0., -1.));
    assert_eq!(comps.normalv, vector(0., 0., -1.));
}

#[test]
fn shading_an_intersection() {
    let w = default_world();
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));

    let i = intersection(4., &w.objects[0]);
    println!("{:?}", i);
    let comps = prepare_computations(&i, &r);
    let hit_color = w.shade_hit(&comps);

    assert_eq!(hit_color, color(0.38066, 0.47583, 0.2855));
}

#[test]
fn shading_an_intersection_from_inside() {
    let w = default_world();
    let r = ray(point(0., 0.25, 0.), vector(1., 1., 1.));

    let i = intersection(0.5, &w.objects[0]);

    let comps = prepare_computations(&i, &r);
    let hit_color = w.shade_hit(&comps);

    assert_eq!(hit_color, color(0.90498, 0.90498, 0.90498));
}

#[test]
fn color_at_when_ray_misses() {
    let w = default_world();
    let r = ray(point(0., 0., -5.), vector(0., 1., 0.));

    let hit_color = w.color_at(&r);

    assert_eq!(hit_color, color(0., 0., 0.));
}

#[test]
fn color_at_when_ray_hits() {
    let w = default_world();
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));

    let hit_color = w.color_at(&r);

    assert_eq!(hit_color, color(0.38066, 0.47583, 0.2855));
}

#[test]
fn color_at_when_intersection_behind_ray() {
    let mut w = default_world();
    let obj1 = &mut w.objects[0];

    obj1.set_material(Material {
        ambient: 1.,
        ..*obj1.material()
    });

    let obj2 = &mut w.objects[1];
    obj2.set_material(Material {
        ambient: 1.,
        ..*obj2.material()
    });

    let r = ray(point(0., 0., 0.75), vector(0., 0., -1.));

    let hit_color = w.color_at(&r);

    assert_eq!(hit_color, w.objects[1].material().color);
}

#[test]

fn when_shade_hit_is_given_intersection_in_shadow() {
    let mut w = default_world();
    w.light = point_light(point(0., 0., -10.), color(1., 1., 1.));

    let s1 = Object::new_sphere();
    let mut s2 = Object::new_sphere();
    s2.set_transform(translation(0., 0., 10.));

    w.objects = vec![s1, s2];
    let r = ray(point(0., 0., 5.), vector(0., 0., 1.));

    let i = intersection(4., &w.objects[1]);

    let comps = prepare_computations(&i, &r);

    let c = w.shade_hit(&comps);

    assert_eq!(c, color(0.1, 0.1, 0.1))
}

#[test]
fn hit_should_offset_the_point() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let mut shape = Object::new_sphere();
    shape.set_transform(translation(0., 0., 1.));

    let i = intersection(5., &shape);
    let comps = prepare_computations(&i, &r);

    assert_eq!(comps.over_point.z < -EPSILON / 2., true);
    assert_eq!(comps.point.z > comps.over_point.z, true);
}
