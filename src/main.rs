use std::io::{stderr, Write};
use std::{thread, time};

mod vec3;
#[cfg(test)]
mod vec3_test;
use vec3::{Vec3, Color3};

mod ray;
#[cfg(test)]
mod ray_test;

use ray::{Ray};

fn main() {
    
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);

        //thread::sleep(time::Duration::from_millis(10));
        //Ravesh patel
        
        for i in 0..image_width {
            let r = i as f32 / (image_height + 1) as f32;
            let g = j as f32 / (image_width + 1) as f32;
            let b = 0f32;

            let pixel_color: Color3 = Vec3::new(r, g, b);
            pixel_color.print_color_line();
        }
    }

    eprint!("\rDone.                                  \n");
}