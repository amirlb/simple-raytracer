use crate::camera::Camera;
use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::geometry::random_unit_vector;
use crate::graphics::Color;
use crate::graphics::BLACK;
use crate::graphics::Image;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use std::io::Write;

pub type ColorMap = dyn Fn(Vec3) -> Color;

pub struct Scene {
    pub camera: Camera,
    pub sky: Box<ColorMap>,
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera {
                position: Vec3(0.0, 0.0, 0.0),
                focal_length: 1.0,
            },
            sky: Box::new(|_| BLACK),
            objects: vec![],
        }
    }

    pub fn add_object(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    fn first_hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut closest = None;
        let mut max = tmax;
        for hittable in self.objects.iter() {
            match hittable.hit(ray, tmin, max) {
                Some(hit_record) => {
                    max = hit_record.t - 0.001;
                    closest = Some(hit_record);
                }
                None => continue
            }
        }
        closest
    }
}

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
            std::io::stdout().flush();
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
            Some(hit_record) => {
                let diffused_ray = Ray {
                    origin: hit_record.hit_point,
                    direction: hit_record.normal + random_unit_vector(),
                };
                let diffused_ray_color = self.ray_color(scene, depth + 1, &diffused_ray);
                Color::mix(BLACK, diffused_ray_color, 0.5)
            }
        }
    }
}
