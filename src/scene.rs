use crate::camera::Camera;
use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::graphics::Color;
use crate::graphics::BLACK;
use crate::material::Material;

pub struct HitRecord {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub type ColorMap = dyn Fn(Vec3) -> Color + Send + Sync;

pub struct SceneObject {
    pub shape: Box<dyn Hittable + Send + Sync>,
    pub material: Material,
}

pub struct Scene {
    pub camera: Camera,
    pub sky: Box<ColorMap>,
    objects: Vec<SceneObject>,
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

    pub fn add_object(&mut self, object: impl Hittable + 'static + Send + Sync, material: Material) {
        self.objects.push(SceneObject {
            shape: Box::new(object),
            material: material,
        });
    }

    pub fn first_hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<(&SceneObject, HitRecord)> {
        let mut closest = None;
        let mut max = tmax;
        for object in self.objects.iter() {
            match object.shape.hit(ray, tmin, max) {
                Some(hit_record) => {
                    max = hit_record.t - 0.001;
                    closest = Some((object, hit_record));
                }
                None => continue
            }
        }
        closest
    }
}
