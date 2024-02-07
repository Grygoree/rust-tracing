use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
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

    pub fn create_unit_vector(self) -> Vec3 { self / self.length() }
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

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]
        }
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

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.x() / rhs, self.y() / rhs, self.z() / rhs]
        }
    }
}

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
