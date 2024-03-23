use rand::{thread_rng, Rng};

pub fn random_float_zero_to_one() -> f32 {
    let mut rng = thread_rng();
    rng.gen()
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float_zero_to_one()
}
