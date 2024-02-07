use crate::Vec3;

#[test]
fn test_new() {
    let subject = Vec3::new(0f32, 1.0, 2.0);
    assert_eq!(subject.x(), 0f32);
}

#[test]
fn test_negation() {
    let subject = -Vec3::new(0f32, 1.0, 2.0);
    assert_eq!(subject.x(), 0f32);
    assert_eq!(subject.y(), -1.0);
    assert_eq!(subject.z(), -2.0);
}

#[test]
fn test_indexing() {
    let subject = Vec3::new(0f32, 1.0, 2.0);
    assert_eq!(subject[0], 0f32);
    assert_eq!(subject[1], 1.0);
    assert_eq!(subject[2], 2.0);
}

#[test]
fn test_mutable_indexing() {
    let mut subject = Vec3::new(0f32, 0f32, 0f32);
    subject[0] = 5.0f32;

    assert_eq!(subject[0], 5.0f32);
}

#[test]
fn test_add_assignment() {
    let mut subject = Vec3::new(1.0, -1.0, 0f32);
    let addend = Vec3::new(3.0, 2.0, 1.0);
    subject += addend;

    assert_eq!(subject.x(), 1.0+3.0);
    assert_eq!(subject.y(), -1.0+2.0);
    assert_eq!(subject.z(), 0f32 + 1.0)
}

#[test]
fn test_mul_assignment_with_vec3() {
    let mut subject = Vec3::new(1.0, -1.0, 0f32);
    let multiplier = Vec3::new(5.0, 1.0, 99.0);
    subject *= multiplier;

    assert_eq!(subject.x(), 1.0*5.0);
    assert_eq!(subject.y(), -1.0*1.0);
    assert_eq!(subject.z(), 0f32);
}

#[test]
fn test_mul_with_f32() {
    let multiplicand = Vec3::new(0f32, 1.0, 2.0);
    let multiplier = -1.0;
    let f32_vec3_subject = multiplicand * multiplier;
    let vec3_f32_subject = multiplier * multiplicand;
    
    assert_eq!(f32_vec3_subject.x(), vec3_f32_subject.x());
    assert_eq!(f32_vec3_subject.x(), 0f32);
}

#[test]
fn test_div_by_f32() {
    let divisor: f32 = 2.;
    let dividend = Vec3::new(0., 1., 2.);
    let quotient = dividend / divisor;

    assert_eq!(quotient.x(), 0.);
    assert_eq!(quotient.y(), 0.5);
    assert_eq!(quotient.z(), 1.);
}