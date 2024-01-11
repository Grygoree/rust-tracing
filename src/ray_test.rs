use crate::{Ray, Vec3};

#[test]
fn test_at() {
    let origin = Vec3::new(0., 0., 0.);
    let direction = Vec3::new(0., 1., 0.);
    let ray = Ray::new(origin, direction);

    let subject = ray.at(2.);

    assert_eq!(subject.x(), 0.);
    assert_eq!(subject.y(), 2.);
}