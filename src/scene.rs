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
    pub sky: Box<ColorMap>,
    objects: Vec<SceneObject>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
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
        let mut closest: Option<(&SceneObject, HitRecord)> = None;
        for object in self.objects.iter() {
            let upper_limit = match &closest {
                None => tmax,
                Some((_, hit_record)) => hit_record.t,
            };
            match object.shape.hit(ray, tmin, upper_limit) {
                Some(hit_record) => { closest = Some((object, hit_record)) }
                None => {}
            };
        }
        closest
    }
}
