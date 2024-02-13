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
use color3::Color3;

fn main() {
    
    // Image

    let aspect_ratio: f32 = 16. / 9.;
    let image_width: i32 = 400;
    // Ensure that image_height is at least 1
    let extant_image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let image_height: i32 = if extant_image_height < 1 { 1 } else { extant_image_height };

    // Camera

    let focal_length: f32 = 1.;
    let viewport_height: f32 = 2.;
    let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::new(0., 0., 0.);

    // Calculate the vectors accross the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel (0,0) in uv-coordinates
    let viewport_upper_left = camera_center - Vec3::new(0.,0.,focal_length) - (viewport_u / 2.) - (viewport_v / 2.);
    let pixel00_loc = viewport_upper_left + (0.5 * (pixel_delta_u + pixel_delta_v));

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);

        //thread::sleep(time::Duration::from_millis(10));
        
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let color = ray_color(r);
            color.print_color_line();
        }
    }

    eprint!("\rDone.                                  \n");
}

/// Returns whether ray r will intersect a sphere of radius radius centered at center
///
/// # Arguments
/// 
/// * `center` - A Vec3 pointer that represents the center of a sphere in 3d space
/// * `radius` - A f32 that represents the radius (distance from center) of the sphere
/// * `r` - A ray pointer that is being tested on whether it intersects the sphere when multiplied by some scalar
fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - *center;
    let a = r.direction.dot(&r.direction);
    let b = 2. * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.*a*c;
    
    if discriminant < 0. { -1. } else {(-b - discriminant.sqrt()) / (2.*a)}
}

fn ray_color(r: Ray) -> Color3 {
    let t = hit_sphere(&Vec3::new(0., 0., -1.), 0.5, &r);
    if t > 0. {
        let n = (r.at(t) - Vec3::new(0.,0.,-1.)).create_unit_vector();
        return 0.5 * Color3(Vec3::new(n.x()+1., n.y()+1., n.z()+1.));
    }

    let unit_direction = r.direction.create_unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    let white = Color3(Vec3::new(1., 1., 1.));
    let blue = Color3(Vec3::new(0.5, 0.7, 1.));
    let end_weight = 1.0 - a;
    end_weight * white + a * blue
}