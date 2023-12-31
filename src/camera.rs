use crate::geometry::Vec3;
use crate::geometry::Ray;
use crate::geometry::cross_product;
use crate::geometry::dot;
use crate::geometry::random_in_unit_circle;
use crate::graphics::Color;
use crate::graphics::BLACK;
use crate::graphics::Image;
use crate::scene::Scene;
use std::io::Write;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;
use num_cpus;

#[derive(Copy, Clone)]
pub struct Bearings {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub up: Vec3,
    pub fov_degrees: f32,
    pub defocus_degrees: f32,
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

fn degrees_to_radians(x: f32) -> f32 {
    x * std::f32::consts::TAU / 360.0
}

#[derive(Copy, Clone)]
pub struct Camera {
    position: Vec3,
    lookat: Vec3,
    right_vector: Vec3,
    up_vector: Vec3,
    defocus_disk_right_vector: Vec3,
    defocus_disk_up_vector: Vec3,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
}

struct LineRenderingResult {
    y: usize,
    line: Vec<Color>,
}

impl Camera {
    pub fn new(bearings: Bearings, image_settings: ImageSettings, render_settings: RenderSettings) -> Camera {
        let image_height = (image_settings.image_width as f32 / image_settings.aspect_ratio).round() as usize;
        let center_vector = bearings.lookat - bearings.lookfrom;
        let focus_distance = center_vector.norm();
        let direction = center_vector / focus_distance;
        let up_vector = (bearings.up - dot(bearings.up, direction) * direction).normalize();
        let right_vector = cross_product(direction, up_vector);
        let pixel_size = degrees_to_radians(0.5 * bearings.fov_degrees).tan() * focus_distance * 2.0 / image_height as f32;
        let defocus_radius = focus_distance * degrees_to_radians(0.5 * bearings.defocus_degrees).tan();
        Camera {
            position: bearings.lookfrom,
            lookat: bearings.lookat,
            right_vector: pixel_size * right_vector,
            up_vector: pixel_size * up_vector,
            defocus_disk_right_vector: defocus_radius * right_vector,
            defocus_disk_up_vector: defocus_radius * up_vector,
            image_width: image_settings.image_width,
            image_height,
            samples_per_pixel: render_settings.samples_per_pixel,
            max_depth: render_settings.max_depth,
        }
    }

    pub fn render(&self, scene: &Scene) -> Image {
        let num_threads = num_cpus::get();
        println!("Rendering on {} threads", num_threads);
        let start_time = Instant::now();
        let image = thread::scope(|scope| {
            let (tx, rx) = mpsc::channel();
            let line_cnt = Arc::new(Mutex::new(0));
            for _ in 0..num_threads {
                let tx = tx.clone();
                let line_cnt = Arc::clone(&line_cnt);
                scope.spawn(move || { self.rendering_thread(scene, tx, line_cnt); });
            }
            self.collect_to_image(rx)
        });
        let runtime = start_time.elapsed().as_nanos() as f64 * 1e-9;
        println!("Finished after {:.1} seconds", runtime);
        image
    }

    fn rendering_thread(&self, scene: &Scene, channel: mpsc::Sender<LineRenderingResult>, line_cnt: Arc<Mutex<usize>>) {
        loop {
            let mut line_to_run = line_cnt.lock().unwrap();
            let y = *line_to_run;
            if y >= self.image_height {
                break
            }
            *line_to_run += 1;
            drop(line_to_run);
            let line = self.render_line(scene, y);
            channel.send(LineRenderingResult{y, line}).unwrap()
        }
    }

    fn collect_to_image(&self, channel: mpsc::Receiver<LineRenderingResult>) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        print!("\rCompleted 0 / {} lines", self.image_height);
        for line_cnt in 0..self.image_height {
            let LineRenderingResult{y, line} = channel.recv().unwrap();
            image.set_line(line, y);
            print!("\rCompleted {} / {} lines", line_cnt + 1, self.image_height);
            std::io::stdout().flush().unwrap();
        }
        println!();

        image
    }

    fn render_line(&self, scene: &Scene, y: usize) -> Vec<Color> {
        (0..self.image_width).map(|x| self.render_pixel(scene, x, y)).collect()
    }

    fn render_pixel(&self, scene: &Scene, x: usize, y: usize) -> Color {
        Color::average((0..self.samples_per_pixel).map(|_| {
            let ray = self.sample_ray_for_pixel(x, y);
            self.ray_color(scene, 0, &ray)
        }))
    }

    fn sample_ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let (fx, fy) = random_in_unit_circle();
        let origin = self.position + fx * self.defocus_disk_right_vector + fy * self.defocus_disk_up_vector;
        let x = x as f32 + rand::random::<f32>() - 0.5 * self.image_width as f32;
        let y = y as f32 + rand::random::<f32>() - 0.5 * self.image_height as f32;
        let destination = self.lookat + x * self.right_vector + y * self.up_vector;
        let ray = Ray {
            origin,
            direction: (destination - origin).normalize(),
        };
        ray
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
