use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::{ray::Ray, vec3::Vec3};
use crate::sphere::Sphere;


#[test]
fn test_sphere_hit() {
    let sphere = build_test_sphere();
    let mut hit_record: HitRecord = Default::default();

    let hitting_ray = Ray::new(Vec3::new(0.,0.,0.), Vec3::new(1., 0., 0.));

    let successful_hit = sphere.hit(&hitting_ray, Interval::new(0., 5.), &mut hit_record);

    assert_eq!(successful_hit, true);
    assert_eq!(hit_record.t, 1.);
    assert_eq!(hit_record.normal.x(), -1.);
    assert_eq!(hit_record.p.x(), 1.);
    assert_eq!(hit_record.front_face, true);
}

#[test]
fn test_sphere_miss() {
    let sphere = build_test_sphere();
    let mut hit_record: HitRecord = Default::default();

    let missing_ray = Ray::new(Vec3::new(0.,0.,0.), Vec3::new(-1., 0., 0.));

    let unsuccessful_hit = sphere.hit(&missing_ray, Interval::new(0., 5.), &mut hit_record);

    assert_eq!(unsuccessful_hit, false);
    // These should be default because a non hit should never modify the hit record
    assert_eq!(hit_record.t, 0.);
    assert_eq!(hit_record.normal.x(), 0.);
    assert_eq!(hit_record.p.x(), 0.);
}

#[test]
fn test_sphere_miss_when_t_max_is_too_small() {
    let sphere = build_test_sphere();
    let mut hit_record: HitRecord = Default::default();

    let missing_ray = Ray::new(Vec3::new(0.,0.,0.), Vec3::new(1., 0., 0.));

    let unsuccessful_hit = sphere.hit(&missing_ray, Interval::new(0., 0.9), &mut hit_record);

    assert_eq!(unsuccessful_hit, false);
    // These should be default because a non hit should never modify the hit record
    assert_eq!(hit_record.t, 0.);
    assert_eq!(hit_record.normal.x(), 0.);
    assert_eq!(hit_record.p.x(), 0.);
}

#[test]
fn test_sphere_miss_when_t_min_is_too_large() {
    let sphere = build_test_sphere();
    let mut hit_record: HitRecord = Default::default();

    let missing_ray = Ray::new(Vec3::new(0.,0.,0.), Vec3::new(1., 0., 0.));

    let unsuccessful_hit = sphere.hit(&missing_ray, Interval::new(4., 5.), &mut hit_record);

    assert_eq!(unsuccessful_hit, false);
    // These should be default because a non hit should never modify the hit record
    assert_eq!(hit_record.t, 0.);
    assert_eq!(hit_record.normal.x(), 0.);
    assert_eq!(hit_record.p.x(), 0.);
}

#[test]
fn test_sphere_hit_when_ray_starts_inside() {
    let sphere = build_test_sphere();
    let mut hit_record: HitRecord = Default::default();

    let hitting_ray = Ray::new(Vec3::new(2.,0.,0.), Vec3::new(1., 0., 0.));

    let successful_hit = sphere.hit(&hitting_ray, Interval::new(0., 3.), &mut hit_record);

    assert_eq!(successful_hit, true);
    assert_eq!(hit_record.t, 1.);
    assert_eq!(hit_record.normal.x(), -1.); // Because it is inside the sphere, the hitting point's normal is opposite the ray
    assert_eq!(hit_record.p.x(), 3.);
    assert_eq!(hit_record.front_face, false); // Inside the sphere, so it is not the front face
}

fn build_test_sphere() -> Sphere {
    Sphere {
        center: Vec3::new(2., 0., 0.),
        radius: 1.
    }
}
