use crate::geometry::Ray;
use crate::graphics::Color;
use crate::graphics::BLACK;
use crate::graphics::Image;
use crate::scene::Scene;
use std::io::Write;

pub struct Tracer {
    pub image_width: usize,
    pub aspect_ratio: f32,
    pub samples_per_pixel: usize,
}

impl Tracer {
    pub fn render(&self, scene: &Scene) -> Image {
        let image_height = (self.image_width as f32 / self.aspect_ratio).round() as usize;
        let mut image = Image::new(self.image_width, image_height);
        for y in 0..image.height {
            for x in 0..image.width {
                let rays = (0..self.samples_per_pixel).map(|_| {
                    let x = x as f32 + rand::random::<f32>();
                    let y = y as f32 + rand::random::<f32>();
                    scene.camera.ray(
                        (2.0 * x - image.width as f32) / image.height as f32,
                        (2.0 * y - image.height as f32) / image.height as f32,
                    )
                });
                *image.at(x, y) = Color::average(rays.map(|ray| self.ray_color(scene, 0, &ray)));
            }
            print!("\rCompleted {} / {} lines", y + 1, image.height);
            std::io::stdout().flush().unwrap()
        }
        println!();
        image
    }

    fn ray_color(&self, scene: &Scene, depth: i32, ray: &Ray) -> Color {
        if depth >= 10 {
            return BLACK;
        }
        match scene.first_hit(ray, 0.001, std::f32::INFINITY) {
            None => {
                (scene.sky)(ray.direction)
            }
            Some((object, hit_record)) => {
                match object.material.scatter(ray, &hit_record) {
                    None => BLACK,
                    Some((attenuation, scattered_ray)) => {
                        let scattered_ray_color = self.ray_color(scene, depth + 1, &scattered_ray);
                        attenuation.attenuate(scattered_ray_color)
                    }
                }
            }
        }
    }
}
