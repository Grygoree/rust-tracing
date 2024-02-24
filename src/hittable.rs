use crate::{vec3::Vec3, Ray};

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t_min: f32, ray_t_max: f32, rec: &mut HitRecord) -> bool;
}