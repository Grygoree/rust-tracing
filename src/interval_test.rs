use crate::interval::Interval;


#[test]
fn test_interval_surrounds() {
    let interval = Interval::new(-1., 1.);

    assert_eq!(interval.surrounds(0.), true);
    assert_eq!(interval.surrounds(-1.), false);
    assert_eq!(interval.surrounds(1.), false);
    assert_eq!(interval.surrounds(2.), false);
}

#[test]
fn test_interval_contains() {
    let interval = Interval::new(-1., 1.);

    assert_eq!(interval.contains(0.), true);
    assert_eq!(interval.contains(-1.), true);
    assert_eq!(interval.contains(1.), true);
    assert_eq!(interval.contains(2.), false);
}

#[test]
fn test_clamp() {
    let interval = Interval::new(-1., 1.);

    assert_eq!(interval.clamp(-3.), -1.);
    assert_eq!(interval.clamp(-1.), -1.);
    assert_eq!(interval.clamp(0.), 0.);
    assert_eq!(interval.clamp(1.), 1.);
    assert_eq!(interval.clamp(2.), 1.);
}

#[test]
fn interval_default() {
    let interval: Interval = Default::default();

    //Contains nothing
    assert_eq!(interval.contains(0.), false);
}
