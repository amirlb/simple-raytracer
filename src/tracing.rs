use crate::camera::Camera;
use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::graphics::Color;
use crate::graphics::BLACK;
use crate::graphics::Image;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;

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

    pub fn render(&self, image_width: usize, aspect_ratio: f32) -> Image {
        let image_height = (image_width as f32 / aspect_ratio).round() as usize;
        let mut image = Image::new(image_width, image_height);
        for y in 0..image.height {
            for x in 0..image.width {
                let ray = self.camera.ray(
                    (2 * x as i32 + 1 - image.width as i32) as f32 / image.height as f32,
                    (2 * y as i32 + 1 - image.height as i32) as f32 / image.height as f32,
                );
                *image.at(x, y) = self.ray_color(&ray);
            }
        }
        image
    }

    fn ray_color(&self, ray: &Ray) -> Color {
        match self.first_hit(ray, 0.001, std::f32::INFINITY) {
            None => {
                (self.sky)(ray.direction)
            }
            Some(hit_record) =>
                Color {
                    red: 0.5*hit_record.normal.0+0.5,
                    green: 0.5*hit_record.normal.1+0.5,
                    blue: 0.5*hit_record.normal.2+0.5,
                }
        }
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
