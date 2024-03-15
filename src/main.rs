mod vec3;
#[cfg(test)]
mod vec3_test;

use vec3::Vec3;

mod ray;
#[cfg(test)]
mod ray_test;
use ray::Ray;

mod color3;
#[cfg(test)]
mod color3_test;

use crate::{camera::Camera, hittable_list::HittableList, sphere::Sphere};

mod hittable;
#[cfg(test)]
mod hittable_test;

mod sphere;
#[cfg(test)]
mod sphere_test;

mod hittable_list;
#[cfg(test)]
mod hittable_list_test;

mod interval;
#[cfg(test)]
mod interval_test;

mod camera;

fn main() {
    // World
    let mut world: HittableList = Default::default();

    world.add(Box::new(Sphere{ center: Vec3::new(0., 0., -1.), radius: 0.5}));
    world.add(Box::new(Sphere{ center: Vec3::new(0., -100.5, -1.), radius: 100.}));

    let mut camera: Camera = Default::default();

    camera.initialize();
    camera.render(world);
}
