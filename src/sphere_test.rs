use crate::hittable::{HitRecord, Hittable};
use crate::{ray::Ray, vec3::Vec3};
use crate::sphere::Sphere;


#[test]
fn test_sphere_hit() {
    let sphere = Sphere {
        center: Vec3::new(2., 0., 0.),
        radius: 1.
    };
    let mut hit_record: HitRecord = Default::default();

    let hitting_ray = Ray::new(Vec3::new(0.,0.,0.), Vec3::new(1., 0., 0.));

    let successful_hit = sphere.hit(&hitting_ray, 0., 5., &mut hit_record);

    assert_eq!(successful_hit, true);
    assert_eq!(hit_record.t, 1.);
    assert_eq!(hit_record.normal.x(), -1.);
    assert_eq!(hit_record.p.x(), 1.);
}

#[test]
fn test_sphere_miss() {
    let sphere = Sphere {
        center: Vec3::new(2., 0., 0.),
        radius: 1.
    };
    let mut hit_record: HitRecord = Default::default();

    let missing_ray = Ray::new(Vec3::new(0.,0.,0.), Vec3::new(-1., 0., 0.));

    let unsuccessful_hit = sphere.hit(&missing_ray, 0., 5., &mut hit_record);

    assert_eq!(unsuccessful_hit, false);
    // These should be default because a non hit should never modify the hit record
    assert_eq!(hit_record.t, 0.);
    assert_eq!(hit_record.normal.x(), 0.);
    assert_eq!(hit_record.p.x(), 0.);
}

#[test]
fn test_sphere_miss_when_t_max_is_too_small() {
    let sphere = Sphere {
        center: Vec3::new(2., 0., 0.),
        radius: 1.
    };
    let mut hit_record: HitRecord = Default::default();

    let missing_ray = Ray::new(Vec3::new(0.,0.,0.), Vec3::new(1., 0., 0.));

    let unsuccessful_hit = sphere.hit(&missing_ray, 0., 0.9, &mut hit_record);

    assert_eq!(unsuccessful_hit, false);
    // These should be default because a non hit should never modify the hit record
    assert_eq!(hit_record.t, 0.);
    assert_eq!(hit_record.normal.x(), 0.);
    assert_eq!(hit_record.p.x(), 0.);
}

#[test]
fn test_sphere_miss_when_t_min_is_too_large() {
    let sphere = Sphere {
        center: Vec3::new(2., 0., 0.),
        radius: 1.
    };
    let mut hit_record: HitRecord = Default::default();

    let missing_ray = Ray::new(Vec3::new(0.,0.,0.), Vec3::new(1., 0., 0.));

    let unsuccessful_hit = sphere.hit(&missing_ray, 4., 5., &mut hit_record);

    assert_eq!(unsuccessful_hit, false);
    // These should be default because a non hit should never modify the hit record
    assert_eq!(hit_record.t, 0.);
    assert_eq!(hit_record.normal.x(), 0.);
    assert_eq!(hit_record.p.x(), 0.);
}
