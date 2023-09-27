use std::io;
mod graphics;
use graphics::Image;
mod geometry;
mod camera;
use camera::Camera;

fn ray_color(r : &geometry::Ray) -> graphics::Color {
    let unit_direction = r.direction.normalize();
    let a = 0.5*(unit_direction.1 + 1.0);
    graphics::Color {
        red: 1.0 - 0.5 * a,
        green: 1.0 - 0.3 * a,
        blue: 1.0,
    }
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio).round() as usize;
    let pixel_size = 2.0 / image_height as f32;

    let mut image = Image::new(image_width, image_height);

    let camera = Camera {
        position: geometry::Vec3(0.0, 0.0, -10.0),
        focal_length: 1.0,
    };
    for y in 0..image.height {
        for x in 0..image.width {
            // *image.at(x, y) = graphics::Color {
            //     red: x as f32 / (image.width - 1) as f32,
            //     green: y as f32 / (image.height - 1) as f32,
            //     blue: 0.0,
            // };
            // if y < image.height / 2 {
            //     *image.at(x, y) = graphics::ORANGE;
            // }
            let ray = camera.ray(
                (2 * x as i32 + 1 - image.width as i32) as f32 / image.height as f32,
                (2 * y as i32 + 1 - image.height as i32) as f32 / image.height as f32,
            );
            *image.at(x, y) = ray_color(&ray);
            // if x > image.width / 2 {
            //     if y%2 == 0 {
            //         *at(&mut image, x, y) = Color {red:1.0, green:1.0, blue:1.0};
            //     }
            // } else {
            //     *at(&mut image, x, y) = Color {red:0.5, green:0.5, blue:0.5};
            // }
        }
    }
    // for i in 0..image.pixels.len() / 2 {
    //     image.pixels[i] = ORANGE;
    // }
    image.save("pic.bmp")?;

    Ok(())
}
