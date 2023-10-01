use crate::geometry::Vec3;
use crate::geometry::Ray;
use crate::geometry::cross_product;
use crate::geometry::dot;
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
pub struct Bearings {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub up: Vec3,
    pub fov_degrees: f32,
}

#[derive(Copy, Clone)]
pub struct ImageSettings {
    pub image_width: usize,
    pub aspect_ratio: f32,
}

#[derive(Copy, Clone)]
pub struct RenderSettings {
    pub samples_per_pixel: usize,
    pub max_depth: usize,
}

impl RenderSettings {
    pub fn shallow() -> RenderSettings {
        RenderSettings {
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }

    pub fn deep() -> RenderSettings {
        RenderSettings {
            samples_per_pixel: 100,
            max_depth: 50,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    position: Vec3,
    direction: Vec3,
    right_vector: Vec3,
    up_vector: Vec3,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Camera {
    pub fn new(bearings: Bearings, image_sett: ImageSettings, render_sett: RenderSettings) -> Camera {
        let image_height = (image_sett.image_width as f32 / image_sett.aspect_ratio).round() as usize;
        let direction = (bearings.lookat - bearings.lookfrom).normalize();
        let up_vector =
            (bearings.up - dot(bearings.up, direction) * direction).normalize() *
            (bearings.fov_degrees * std::f32::consts::TAU / 720.0).tan() *
            2.0 / image_height as f32;
        let right_vector = cross_product(direction, up_vector);
        Camera {
            position: bearings.lookfrom,
            direction,
            right_vector,
            up_vector,
            image_width: image_sett.image_width,
            image_height,
            samples_per_pixel: render_sett.samples_per_pixel,
            max_depth: render_sett.max_depth,
        }
    }

    pub fn render(&self, scene: Scene) -> Image {
        let scene = Arc::new(scene);
        let camera = Arc::new(*self);
        let mut image = Image::new(self.image_width, self.image_height);
        print!("\rCompleted 0 / {} lines", self.image_height);
        let pool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = mpsc::channel();
        for y in 0..self.image_height {
            let tx = tx.clone();
            let scene = Arc::clone(&scene);
            let camera = Arc::clone(&camera);
            pool.execute(move || {
                let line = camera.render_line(&scene, y);
                tx.send((y, line)).unwrap()
            });
        }
        for line_cnt in 0..self.image_height {
            let (y, line) = rx.recv().unwrap();
            image.set_line(line, y);
            print!("\rCompleted {} / {} lines", line_cnt + 1, self.image_height);
            std::io::stdout().flush().unwrap()
        }
        println!();
        image
    }

    fn render_line(&self, scene: &Scene, y: usize) -> Vec<Color> {
        (0..self.image_width).map(|x| self.render_pixel(scene, x, y)).collect()
    }

    fn render_pixel(&self, scene: &Scene, x: usize, y: usize) -> Color {
        let rays = (0..self.samples_per_pixel).map(|_| {
            let x = x as f32 + rand::random::<f32>() - 0.5 * self.image_width as f32;
            let y = y as f32 + rand::random::<f32>() - 0.5 * self.image_height as f32;
            Ray {
                origin: self.position,
                direction: self.direction + x * self.right_vector + y * self.up_vector,
            }
        });
        Color::average(rays.map(|ray| self.ray_color(scene, 0, &ray)))
    }

    fn ray_color(&self, scene: &Scene, depth: usize, ray: &Ray) -> Color {
        if depth >= self.max_depth {
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
