use crate::{vec3::Vec3, Ray};

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t_min: f32, ray_t_max: f32, rec: &mut HitRecord) -> bool;
}