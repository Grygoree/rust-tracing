use std::ops::{Add, Neg, Index, IndexMut, AddAssign, MulAssign, Mul};

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

pub type Color3 = (Vec3);

impl Color3 {
    pub fn print_color_line(&self) {
        let byte_scaled_color = self * 255.999f32;
        let r = byte_scaled_color[0] as i32;
        let g = byte_scaled_color[1] as i32;
        let b = byte_scaled_color[2] as i32;

        println!("{} {} {}", r, g, b);
    }
}



impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            e: [x, y, z]
        }
    }

    pub(crate) fn x(&self) -> f32 { self.e[0] }
    pub(crate) fn y(&self) -> f32 { self.e[1] }
    pub(crate) fn z(&self) -> f32 { self.e[2] }

    pub(crate) fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub(crate) fn length_squared(&self) -> f32 {
        (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z())
    }

    pub(crate) fn lerp(&self, start: f32, scale: f32) -> Vec3 {
        self*scale + start
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.x() + rhs, self.y() + rhs, self.z() + rhs]
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() +rhs.z()]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.x()*rhs, self.y()*rhs, self.z()*rhs]
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.x()*rhs, self.y()*rhs, self.z()*rhs]
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [rhs.x()*self, rhs.y()*self, rhs.z()*self]
        }
    }
}

