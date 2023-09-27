use std::io;
mod graphics;
use graphics::Image;

fn main() -> io::Result<()> {
    let mut image = Image::new(400, 300);
    for y in 0..image.height {
        for x in 0..image.width {
            *image.at(x, y) = graphics::Color {
                red: x as f32 / (image.width - 1) as f32,
                green: y as f32 / (image.height - 1) as f32,
                blue: 0.0,
            };
            if y < image.height / 2 {
                *image.at(x, y) = graphics::ORANGE;
            }
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
