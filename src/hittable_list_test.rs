use crate::{hittable::{HitRecord, Hittable}, hittable_list::HittableList, ray::Ray, sphere::Sphere, vec3::Vec3};

#[test]
fn test_hittable_list_add_and_hit() {
    let sphere1 = Sphere {
        center: Vec3::new(2., 0., 0.),
        radius: 1.
    };
    let sphere2 = Sphere {
        center: Vec3::new(3., 0., 0.),
        radius: 1.
    };
    let ray = Ray {
        origin: Vec3::new(0., 0., 0.),
        direction: Vec3::new(1., 0., 0.)
    };
    let mut rec: HitRecord = Default::default();
    let mut list = HittableList::new();
    
    list.add(Box::new(sphere1));
    list.add(Box::new(sphere2));

    let result = list.hit(&ray, 0., f32::INFINITY, &mut rec);

    assert_eq!(result, true);
    assert_eq!(rec.p.x(), 1.);
}

#[test]
fn test_hittable_list_clear() {
    let sphere1 = Sphere {
        center: Vec3::new(1., 0., 0.),
        radius: 1.
    };
    let ray = Ray {
        origin: Vec3::new(0., 0., 0.),
        direction: Vec3::new(1., 0., 0.)
    };
    let mut rec: HitRecord = Default::default();
    let mut list = HittableList::new();
    
    list.add(Box::new(sphere1));
    list.clear();

    let result = list.hit(&ray, 0., f32::INFINITY, &mut rec);

    assert_eq!(result, false);
}
