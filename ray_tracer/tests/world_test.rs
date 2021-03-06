use ray_tracer::{
    color::color,
    light::point_light,
    material::Material,
    pattern::test_pattern,
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
    assert!(w.objects[0] == obj);
}

#[test]
fn default_world_has_lighting_and_2_spheres() {
    let world = default_world();

    assert_eq!(world.light.position, point(-10., 10., -10.));
    assert_eq!(world.light.intensity, color(1., 1., 1.));

    assert_eq!(world.objects[0].material.color, color(0.8, 1., 0.6));
    assert_eq!(world.objects[0].material.diffuse, 0.7);
    assert_eq!(world.objects[0].material.specular, 0.2);

    assert_eq!(world.objects[1].transform, scaling(0.5, 0.5, 0.5));
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
    let xs = Intersect { locations: vec![i] };

    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    assert_eq!(comps.t, 4.);
    assert_eq!(comps.object.uuid, shape.uuid);
    assert_eq!(comps.point, point(0., 0., -1.));
    assert_eq!(comps.eyev, vector(0., 0., -1.));
    assert_eq!(comps.normalv, vector(0., 0., -1.));
}

#[test]
fn pre_computing_state_of_intersection_when_outside() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let shape = Object::new_sphere();
    let i = intersection(4., &shape);
    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    assert_eq!(comps.inside, false);
}

#[test]
fn pre_computing_state_of_intersection_when_inside() {
    let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
    let shape = Object::new_sphere();
    let i = intersection(1., &shape);
    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
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
    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    let hit_color = w.shade_hit(&comps, 1);

    assert_eq!(hit_color, color(0.38066, 0.47583, 0.2855));
}

#[test]
fn shading_an_intersection_from_inside() {
    let mut w = default_world();
    w.light = point_light(point(0., 0.25, 0.), color(1., 1., 1.));
    let r = ray(point(0., 0., 0.), vector(0., 0., 1.));

    let i = intersection(0.5, &w.objects[1]);

    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    let hit_color = w.shade_hit(&comps, 1);

    assert_eq!(hit_color, color(0.90498, 0.90498, 0.90498));
}

#[test]
fn color_at_when_ray_misses() {
    let w = default_world();
    let r = ray(point(0., 0., -5.), vector(0., 1., 0.));

    let hit_color = w.color_at(&r, 1);

    assert_eq!(hit_color, color(0., 0., 0.));
}

#[test]
fn color_at_when_ray_hits() {
    let w = default_world();
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));

    let hit_color = w.color_at(&r, 1);

    assert_eq!(hit_color, color(0.38066, 0.47583, 0.2855));
}

#[test]
fn color_at_when_intersection_behind_ray() {
    let mut w = default_world();
    let obj1 = &mut w.objects[0];

    obj1.set_material(Material {
        ambient: 1.,
        pattern: None,
        ..obj1.material
    });

    let obj2 = &mut w.objects[1];
    obj2.set_material(Material {
        ambient: 1.,
        pattern: None,
        ..obj2.material
    });

    let r = ray(point(0., 0., 0.75), vector(0., 0., -1.));

    let hit_color = w.color_at(&r, 1);

    assert_eq!(hit_color, w.objects[1].material.color);
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

    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);

    let c = w.shade_hit(&comps, 1);

    assert_eq!(c, color(0.1, 0.1, 0.1))
}

#[test]
fn hit_should_offset_the_point() {
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let mut shape = Object::new_sphere();
    shape.set_transform(translation(0., 0., 1.));

    let i = intersection(5., &shape);
    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);

    assert_eq!(comps.over_point.z < (-EPSILON / 2.), true);
    assert_eq!(comps.point.z > comps.over_point.z, true);
}

// Reflection
#[test]
fn pre_computing_the_reflection_vector() {
    let square_root_of_2 = (2. as f64).sqrt();
    let shape = Object::new_plane();
    let r = ray(
        point(0., 1., -1.),
        vector(0., -square_root_of_2 / 2., square_root_of_2 / 2.),
    );

    let i = intersection(square_root_of_2, &shape);

    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);

    assert_eq!(
        comps.reflectv,
        vector(0., square_root_of_2 / 2., square_root_of_2 / 2.)
    );
}

#[test]
fn reflected_color_for_nonreflective_material() {
    let w = &mut default_world();
    let r = ray(point(0., 0., 0.), vector(0., 0., 1.));

    {
        let shape = &mut w.objects[1];
        shape.material.ambient = 0.64;
    }

    let i = intersection(1., &w.objects[1]);

    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);

    assert_eq!(w.reflected_color(&comps, 1), color(0., 0., 0.));
}

#[test]
fn reflected_color_for_reflective_material() {
    let square_root_of_2 = (2. as f64).sqrt();
    let mut w = default_world();

    let mut shape = Object::new_plane();
    shape.material.reflective = 0.5;
    shape.transform = translation(0., -1., 0.);

    w.objects.push(shape);
    let r = ray(
        point(0., 0., -3.),
        vector(0., -square_root_of_2 / 2., square_root_of_2 / 2.),
    );

    let i = intersection(square_root_of_2, &w.objects.last().unwrap());

    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);

    // r: 0.19032 -> 0.19033
    assert_eq!(
        w.reflected_color(&comps, 1),
        color(0.19033, 0.23791, 0.14274)
    );
}

#[test]
fn shade_hit_with_reflective_material() {
    let square_root_of_2 = (2. as f64).sqrt();
    let mut w = default_world();

    let mut shape = Object::new_plane();
    shape.material.reflective = 0.5;
    shape.set_transform(translation(0., -1., 0.));

    w.objects.push(shape);

    let r = ray(
        point(0., 0., -3.),
        vector(0., -square_root_of_2 / 2., square_root_of_2 / 2.),
    );

    let i = intersection(square_root_of_2, &w.objects.last().unwrap());

    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);

    // bumped decimal points provided by book
    assert_eq!(w.shade_hit(&comps, 5), color(0.87676, 0.92434, 0.82917));
}

#[test]
fn color_at_with_mutually_reflective_surfaces() {
    let mut w = default_world();
    w.light = point_light(point(0., 0., 0.), color(1., 1., 1.));
    {
        let mut lower = Object::new_plane();
        lower.material.reflective = 1.;
        lower.transform = translation(0., -1., 0.);

        let mut upper = Object::new_plane();
        upper.material.reflective = 1.;
        upper.transform = translation(0., 1., 0.);

        w.objects = vec![lower, upper];
    }

    let r = ray(point(0., 0., -3.), vector(0., 1., 0.));

    w.color_at(&r, 1);
}

#[test]
fn the_reflected_color_at_maximum_recursive_depth() {
    let square_root_of_2 = (2. as f64).sqrt();

    let mut w = default_world();
    {
        let mut shape = Object::new_plane();
        shape.material.reflective = 0.5;
        shape.transform = translation(0., -1., 0.);
        w.objects.push(shape);
    }

    let r = ray(
        point(0., 0., -3.),
        vector(0., -square_root_of_2 / 2., square_root_of_2 / 2.),
    );
    let i = intersection(square_root_of_2, w.objects.last().unwrap());

    let xs = Intersect { locations: vec![i] };
    let comps = prepare_computations(&xs.locations[0], &r, &xs);

    let c = w.reflected_color(&comps, 0);

    assert_eq!(c, color(0., 0., 0.))
}

#[test]
fn refracted_color_with_opaque_surface() {
    let w = default_world();
    let shape = &w.objects[0];
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let xs = intersections(vec![intersection(4., shape), intersection(6., shape)]);
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    let c = w.refracted_color(&comps, 5);
    assert_eq!(c, color(0., 0., 0.))
}

#[test]
fn refracted_color_at_maximum_recursive_depth() {
    let mut w = default_world();
    {
        let shape = &mut w.objects[0];
        shape.material.transparency = 1.;
        shape.material.refractive_index = 1.5;
    }
    let shape = &w.objects[0];
    let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
    let xs = intersections(vec![intersection(4., shape), intersection(6., shape)]);
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    let c = w.refracted_color(&comps, 0);
    assert_eq!(c, color(0., 0., 0.))
}

#[test]
fn refracted_color_under_total_internal_reflection() {
    let square_root_of_2 = (2. as f64).sqrt();
    let mut w = default_world();
    {
        let shape = &mut w.objects[0];
        shape.material.transparency = 1.;
        shape.material.refractive_index = 1.5;
    }
    let shape = &w.objects[0];
    let r = ray(point(0., 0., square_root_of_2 / 2.), vector(0., 1., 0.));
    let xs = intersections(vec![
        intersection(-square_root_of_2 / 2., shape),
        intersection(square_root_of_2 / 2., shape),
    ]);
    let comps = prepare_computations(&xs.locations[1], &r, &xs);
    let c = w.refracted_color(&comps, 5);
    assert_eq!(c, color(0., 0., 0.))
}

#[test]
fn refracted_color_with_refracted_ray() {
    let mut w = default_world();

    {
        let a = &mut w.objects[0];
        a.material.ambient = 1.0;
        a.material.pattern = Some(Box::new(test_pattern()));
    }

    {
        let b = &mut w.objects[1];
        b.material.transparency = 1.0;
        b.material.refractive_index = 1.5;
    }

    let r = ray(point(0., 0., 0.1), vector(0., 1., 0.));
    let xs = intersections(vec![
        intersection(-0.9899, &w.objects[0]),
        intersection(-0.4899, &w.objects[1]),
        intersection(0.4899, &w.objects[1]),
        intersection(0.9899, &w.objects[0]),
    ]);
    let comps = prepare_computations(&xs.locations[2], &r, &xs);
    let c = w.refracted_color(&comps, 5);

    assert_eq!(c, color(0., 0.99888, 0.04722))
}

#[test]
fn shade_hit_with_transparent_material() {
    let square_root_of_2 = (2. as f64).sqrt();
    let mut w = default_world();

    let mut floor = Object::new_plane();
    floor.set_transform(translation(0., -1., 0.));
    floor.material.transparency = 0.5;
    floor.material.refractive_index = 1.5;

    let mut ball = Object::new_sphere();
    ball.material.color = color(1., 0., 0.);
    ball.material.ambient = 0.5;
    ball.set_transform(translation(0., -3.5, -0.5));

    w.objects.push(ball);
    w.objects.push(floor);

    let r = ray(
        point(0., 0., -3.),
        vector(0., -square_root_of_2 / 2., square_root_of_2 / 2.),
    );
    let xs = intersections(vec![intersection(
        square_root_of_2,
        w.objects.last().unwrap(),
    )]);
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    let c = w.shade_hit(&comps, 5);
    assert_eq!(c, color(0.93642, 0.68642, 0.68642))
}

#[test]
fn schlick_approximation_under_total_internal_reflection() {
    let square_root_of_2 = (2. as f64).sqrt();
    let shape = Object::new_glass_sphere();
    let r = ray(point(0., 0., square_root_of_2 / 2.), vector(0., 1., 0.));
    let xs = intersections(vec![
        intersection(-square_root_of_2 / 2., &shape),
        intersection(square_root_of_2 / 2., &shape),
    ]);
    let comps = prepare_computations(&xs.locations[1], &r, &xs);
    let reflectance = schlick(&comps);
    assert_eq!(reflectance, 1.0)
}

#[test]
fn schlick_approximation_with_perpendicular_viewing_angle() {
    let shape = Object::new_glass_sphere();
    let r = ray(point(0., 0., 0.), vector(0., 1., 0.));
    let xs = intersections(vec![intersection(-1., &shape), intersection(1., &shape)]);
    let comps = prepare_computations(&xs.locations[1], &r, &xs);
    let reflectance = schlick(&comps);
    assert!((reflectance - 0.04).abs() < EPSILON)
}

#[test]
fn schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
    let shape = Object::new_glass_sphere();
    let r = ray(point(0., 0.99, -2.), vector(0., 0., 1.));
    let xs = intersections(vec![intersection(1.8589, &shape)]);
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    let reflectance = schlick(&comps);
    assert!((reflectance - 0.48873).abs() < EPSILON)
}

#[test]
fn shade_hit_with_a_reflective_and_transparent_material() {
    let square_root_of_2 = (2. as f64).sqrt();
    let mut w = default_world();

    let mut floor = Object::new_plane();
    floor.set_transform(translation(0., -1., 0.));
    floor.material.reflective = 0.5;
    floor.material.transparency = 0.5;
    floor.material.refractive_index = 1.5;

    let mut ball = Object::new_sphere();
    ball.material.color = color(1., 0., 0.);
    ball.material.ambient = 0.5;
    ball.set_transform(translation(0., -3.5, -0.5));

    w.objects.push(ball);
    w.objects.push(floor);

    let r = ray(
        point(0., 0., -3.),
        vector(0., -square_root_of_2 / 2., square_root_of_2 / 2.),
    );
    let xs = intersections(vec![intersection(
        square_root_of_2,
        w.objects.last().unwrap(),
    )]);
    let comps = prepare_computations(&xs.locations[0], &r, &xs);
    let c = w.shade_hit(&comps, 5);
    assert_eq!(c, color(0.93391, 0.69643, 0.69243))
}
