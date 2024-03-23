use crate::{color3::Color3, hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, utils::random_float_zero_to_one, vec3::Vec3};

#[derive(Default)]
pub struct Camera {
    aspect_ratio: f32,
    image_width: u16,

    image_height: u16,
    center: Vec3,
    pixel00_loc: Vec3, // Location of pixel 0,0 in worldspace
    pixel_delta_u: Vec3, // Right offset pixel
    pixel_delta_v: Vec3, // Below offset pixel
    samples_per_pixel: u16,
}

impl Camera {
    pub fn render(self, world : impl Hittable) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);

            for i in 0..self.image_width {
                let mut pixel_color = Color3(Vec3::new(0.,0.,0.));
                for _ in 0..=self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, &world);
                }
                pixel_color.print_color_line(self.samples_per_pixel);
            }
        }
    }

    pub fn initialize(&mut self) {
        self.aspect_ratio = 16. / 9.;
        self.image_width = 400;
        self.samples_per_pixel = 10;

        let extant_image_height = (self.image_width as f32 / self.aspect_ratio) as u16;
        self.image_height = if extant_image_height < 1 { 1 } else { extant_image_height };

        self.center = Vec3::new(0.,0.,0.);

        // Determine viewport dimensions
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - Vec3::new(0., 0., focal_length) - (viewport_u / 2.) - (viewport_v / 2.);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    /// Gets a randomly sampled ray at location [i](u16), [j](u16)
    fn get_ray(&self, i: u16, j: u16) -> Ray {
       let pixel_center = self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
       let pixel_sample = pixel_center + self.pixel_sample_square();

       let ray_origin = self.center;
       let ray_direction = pixel_sample - ray_origin;
 
       Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_float_zero_to_one();
        let py = -0.5 + random_float_zero_to_one();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn ray_color(&self, ray: Ray, world: &impl Hittable) -> Color3 {
        let mut rec: HitRecord = Default::default();

        if world.hit(&ray, Interval::new(0., f32::INFINITY), &mut rec) {
            return 0.5 * (Color3(rec.normal) + Color3(Vec3::new(1., 1., 1.)))
        }

        // Linearly interpolate between light blue and white based on the y component of ray
        let unit_direction = ray.direction.create_unit_vector();
        let lerp_t = 0.5 * unit_direction.y() + 1.;
        (1. - lerp_t) * Color3(Vec3::new(1.,1.,1.)) + lerp_t * (Color3(Vec3::new(0.5, 0.7, 1.0)))
    }
}
