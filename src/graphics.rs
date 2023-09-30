use std::io;
use std::io::Write;
use std::fs::File;

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn mix(color1: Color, color2: Color, t: f32) -> Color {
        Color {
            red: (1.0-t) * color1.red + t * color2.red,
            green: (1.0-t) * color1.green + t * color2.green,
            blue: (1.0-t) * color1.blue + t * color2.blue,
        }
    }

    pub fn attenuate(&self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }

    pub fn average(iter: impl Iterator<Item=Color>) -> Color {
        let mut count = 0;
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;
        for color in iter {
            count += 1;
            red += color.red;
            green += color.green;
            blue += color.blue;
        }
        Color {
            red: red / count as f32,
            green: green / count as f32,
            blue: blue / count as f32,
        }
    }
}

pub const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0 };
pub const WHITE: Color = Color { red: 1.0, green: 1.0, blue: 1.0 };

pub struct Image {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

fn set_u32(arr: &mut [u8], ind: usize, value: u32) {
    arr[ind] = (value & 0xFF) as u8;
    arr[ind+1] = ((value>>8) & 0xFF) as u8;
    arr[ind+2] = ((value>>16) & 0xFF) as u8;
    arr[ind+3] = ((value>>24) & 0xFF) as u8;
}

fn float_to_u8(x: f32) -> u8 {
    if x < 0.0 {
        return 0
    }
    if x > 1.0 {
        return 255
    }
    return (255.0 * x).round() as u8
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width: width,
            height: height,
            pixels: vec![BLACK; width * height],
        }
    }

    pub fn at(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.pixels[y * self.width + x]
    }

    pub fn save(&self, file_name: &str) -> io::Result<()> {
        let mut f = File::create(file_name)?;
        const HEADER_SIZE: usize = 140;
        let line_bytes = self.width * 3 + self.width % 4;
        let data_size = line_bytes * self.height;
        let mut header: [u8; HEADER_SIZE] = [0; HEADER_SIZE];
        header[0] = b'B';
        header[1] = b'M';
        set_u32(&mut header, 2, (HEADER_SIZE + data_size) as u32);
        set_u32(&mut header, 10, HEADER_SIZE as u32);
        set_u32(&mut header, 14, 0x7c); // Win5xBitmapHeader
        set_u32(&mut header, 18, self.width as u32);
        set_u32(&mut header, 22, self.height as u32);
        header[26] = 1;  // numColorPlanes
        header[28] = 24;  // bitsPerPixel
        f.write(&header)?;
        let mut data = vec![0; data_size];
        const INV_GAMMA: f32 = 0.45;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_ind = y * self.width + x;
                let data_ind = y * line_bytes + x * 3;
                data[data_ind] = float_to_u8(f32::powf(self.pixels[pixel_ind].blue, INV_GAMMA));
                data[data_ind + 1] = float_to_u8(f32::powf(self.pixels[pixel_ind].green, INV_GAMMA));
                data[data_ind + 2] = float_to_u8(f32::powf(self.pixels[pixel_ind].red, INV_GAMMA));
            }
        }
        f.write(&data)?;
        Ok(())
    }
}
