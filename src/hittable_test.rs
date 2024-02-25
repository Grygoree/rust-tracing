use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};


#[test]
fn test_hit_face_normal_from_outside() {
    let mut hit_record: HitRecord = Default::default();
    let outward_normal = Vec3::new(-1., 0., 0.);
    let ray = Ray {
        origin: Default::default(),
        direction: Vec3::new(1., 0., 0.),
    };

    hit_record.set_face_normal(&ray, &outward_normal);

    assert_eq!(hit_record.front_face, true);
    assert_eq!(hit_record.normal.x(), -1.);
}

#[test]
fn test_hit_face_normal_from_inside() {
    let mut hit_record: HitRecord = Default::default();
    let outward_normal = Vec3::new(1., 0., 0.);
    let ray = Ray {
        origin: Default::default(),
        direction: Vec3::new(1., 0., 0.),
    };

    hit_record.set_face_normal(&ray, &outward_normal);

    assert_eq!(hit_record.front_face, false);
    assert_eq!(hit_record.normal.x(), -1.);
}
