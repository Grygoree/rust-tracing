use crate::{hittable::{HitRecord, Hittable}, ray::Ray};

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            list: Vec::new()
        }
    }

    pub fn clear(& mut self) { self.list.clear() }

    pub fn add(& mut self, obj: Box<dyn Hittable>) { self.list.push(obj)}
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t_min: f32, ray_t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything_so_far = false;
        let mut closest_so_far = ray_t_max;

        for hittable in &self.list {
            if hittable.hit(r, ray_t_min, closest_so_far, &mut temp_rec) {
                hit_anything_so_far = true;
                closest_so_far = temp_rec.t;
                
                // TODO: Ask rust friends to rusty way of doing this
                // rec = temp_rec 
                rec.front_face = temp_rec.front_face;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
            }
        }

        hit_anything_so_far
    }
}
