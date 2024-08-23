mod vec3;
mod color;

use vec3::Vec3;
use color::Color;


fn main() {
    let image_width = 256;
    let image_height = 256;


    println!("P3\n{} {} \n255\n", image_width, image_height);
    
    for i in 0..image_height {
        for j in 0..image_width{
            let r: f32 = (i as f32) / (image_width - 1) as f32;
            let g: f32 = (j as f32) / (image_height - 1) as f32;
            let b: f32 = 0.0;
            let pixel_color = Color::new(r, g, b);    

            println!("{}", pixel_color);
        
        }
    }

}
