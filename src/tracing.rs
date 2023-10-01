use crate::geometry::Ray;
use crate::graphics::Color;
use crate::graphics::BLACK;
use crate::graphics::Image;
use crate::scene::Scene;
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc;
use threadpool::ThreadPool;
use num_cpus;

#[derive(Copy, Clone)]
pub struct Tracer {
    pub image_width: usize,
    pub aspect_ratio: f32,
    pub samples_per_pixel: usize,
}

impl Tracer {
    pub fn render(&self, scene: Arc<Scene>) -> Image {
        let tracer = Arc::new(*self);
        let image_height = (self.image_width as f32 / self.aspect_ratio).round() as usize;
        let mut image = Image::new(self.image_width, image_height);
        print!("\rCompleted 0 / {} lines", image.height);
        let pool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = mpsc::channel();
        for y in 0..image.height {
            let tx = tx.clone();
            let scene = Arc::clone(&scene);
            let tracer = Arc::clone(&tracer);
            pool.execute(move || {
                let line = tracer.render_line(&scene, image.width, image.height, y);
                tx.send((y, line)).unwrap()
            });
        }
        for line_cnt in 0..image.height {
            let (y, line) = rx.recv().unwrap();
            image.set_line(line, y);
            print!("\rCompleted {} / {} lines", line_cnt + 1, image.height);
            std::io::stdout().flush().unwrap()
        }
        println!();
        image
    }

    fn render_line(&self, scene: &Scene, image_width: usize, image_height: usize, y: usize) -> Vec<Color> {
        (0..image_width).map(|x| self.render_pixel(scene, image_width, image_height, x, y)).collect()
    }

    fn render_pixel(&self, scene: &Scene, image_width: usize, image_height: usize, x: usize, y: usize) -> Color {
        let rays = (0..self.samples_per_pixel).map(|_| {
            let x = x as f32 + rand::random::<f32>();
            let y = y as f32 + rand::random::<f32>();
            scene.camera.ray(
                (2.0 * x - image_width as f32) / image_height as f32,
                (2.0 * y - image_height as f32) / image_height as f32,
            )
        });
        Color::average(rays.map(|ray| self.ray_color(scene, 0, &ray)))
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
