use crate::camera::Camera;
use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::graphics::Color;
use crate::graphics::BLACK;

pub struct HitRecord {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

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

    pub fn first_hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
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
