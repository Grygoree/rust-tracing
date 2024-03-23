use crate::{interval::Interval, Vec3};
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy)]
pub struct Color3(pub Vec3);

impl Color3 {
    pub fn print_color_line(&self, samples_per_pixel: u16) {
        let scale = 1. / samples_per_pixel as f32;
        let scaled_color = self.0 * scale * 256.;
        let r = scaled_color[0];
        let g = scaled_color[1];
        let b = scaled_color[2];

        let intensity = Interval::new(0., 0.999);
        println!("{} {} {}", intensity.clamp(r), intensity.clamp(g), intensity.clamp(b));
    }
}

impl Add for Color3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color3(self.0 + rhs.0)
    }
}

impl AddAssign for Color3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul<f32> for Color3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color3(self.0 * rhs)
    }
}

impl Mul<Color3> for f32 {
    type Output = Color3;

    fn mul(self, rhs: Color3) -> Self::Output {
        Color3(rhs.0 * self)
    }
}
