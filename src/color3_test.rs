use crate::{Color3, Vec3};

#[test]
fn test_color_addition() {
    let red = Color3(Vec3::new(1., 0., 0.));
    let blue = Color3(Vec3::new(0., 0., 1.));

    let purple = red + blue;

    assert_eq!(purple.0.x(), 1.);
    assert_eq!(purple.0.z(), 1.);
}

#[test]
fn test_color_scalar_multiplication() {
    let grey = Color3(Vec3::new(0.5, 0.5, 0.5));
    let white = 2. * grey;
    let another_white = grey * 2.;

    assert_eq!(1., white.0.x());
    assert_eq!(white.0.x(), another_white.0.x());
}
