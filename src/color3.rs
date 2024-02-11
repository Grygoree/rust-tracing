use crate::Vec3;
use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Color3(pub Vec3);

impl Color3 {
    pub fn print_color_line(&self) {
        let byte_scaled_color = self.0 * 255.999f32;
        let r = byte_scaled_color[0] as i32;
        let g = byte_scaled_color[1] as i32;
        let b = byte_scaled_color[2] as i32;

        println!("{} {} {}", r, g, b);
    }
}

impl Add for Color3 {
    type Output =  Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color3(self.0 + rhs.0)
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
