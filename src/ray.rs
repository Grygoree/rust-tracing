use crate::Vec3;

#[derive(Default, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction
        }
    }
    
    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + (t * self.direction)
    }
}