use crate::geometry::Vec3;
use crate::geometry::Ray;
use crate::geometry::cross_product;
use crate::geometry::dot;
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

fn sample_antialias_filter() -> (f32, f32, f32) {
    // None
    // (0.5, 0.5, 1.0)

    // Square
    (rand::random::<f32>(), rand::random::<f32>(), 1.0)

    // Gaussian
    // let u = 1.0 - rand::random::<f32>();
    // let r = -0.5 * u.ln();
    // let theta = std::f32::consts::TAU * rand::random::<f32>();
    // (r * theta.cos() + 0.5, r * theta.sin() + 0.5, 1.0)

    // Mitchell
    // if x < 1 {
    //     7/6 x^3 - 2 x^2 + 8/9
    // } else if x < 2 {
    //     -7/18 x^3 + 2 x^2 - 10/3 x + 16/9
    // } else {
    //     0
    // }
}

struct LineRenderingResult {
    y: usize,
    line: Vec<Color>,
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
            // drop(line_to_run);
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
        let mut sum_red = 0.0;
        let mut sum_green = 0.0;
        let mut sum_blue = 0.0;
        for _ in 0..self.samples_per_pixel {
            let (ray, weight) = self.sample_ray_for_pixel(x, y);
            let Color{red, green, blue} = self.ray_color(scene, 0, &ray);
            sum_red += weight * red;
            sum_green += weight * green;
            sum_blue += weight * blue;
        }
        let factor = 1.0 / self.samples_per_pixel as f32;
        Color {
            red: factor * sum_red,
            green: factor * sum_green,
            blue: factor * sum_blue,
        }
    }

    fn sample_ray_for_pixel(&self, x: usize, y: usize) -> (Ray, f32) {
        let (dx, dy, weight) = sample_antialias_filter();
        let x = x as f32 + dx - 0.5 * self.image_width as f32;
        let y = y as f32 + dy - 0.5 * self.image_height as f32;
        let ray = Ray {
            origin: self.position,
            direction: (self.direction + x * self.right_vector + y * self.up_vector).normalize(),
        };
        (ray, weight)
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
