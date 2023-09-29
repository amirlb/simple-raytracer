use crate::camera::Camera;
use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::graphics::Color;
use crate::graphics::BLACK;
use crate::graphics::Image;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use rand;

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
        for hittable in &self.objects {
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
                let mut red = 0.0;
                let mut green = 0.0;
                let mut blue = 0.0;
                for ray in rays {
                    let color = self.ray_color(scene, &ray);
                    red += color.red;
                    green += color.green;
                    blue += color.blue;
                }
                *image.at(x, y) = Color {
                    red: red / self.samples_per_pixel as f32,
                    green: green / self.samples_per_pixel as f32,
                    blue: blue / self.samples_per_pixel as f32,
                };
            }
        }
        image
    }

    fn ray_color(&self, scene: &Scene, ray: &Ray) -> Color {
        match scene.first_hit(ray, 0.001, std::f32::INFINITY) {
            None => {
                (scene.sky)(ray.direction)
            }
            Some(hit_record) =>
                Color {
                    red: 0.5*hit_record.normal.0+0.5,
                    green: 0.5*hit_record.normal.1+0.5,
                    blue: 0.5*hit_record.normal.2+0.5,
                }
        }
    }
}
