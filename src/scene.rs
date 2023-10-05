use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::graphics::Color;
use crate::graphics::BLACK;

pub struct HitRecord {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
    // TODO: separate hittable from shape
    fn contains(&self, point: Vec3) -> bool;
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

// TODO: switch to take ray as input
pub type ColorMap = dyn Fn(Vec3) -> Color + Send + Sync;

pub struct SceneObject {
    pub shape: Box<dyn Hittable + Send + Sync>,
    pub material: Box<dyn Material + Send + Sync>,
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

    pub fn add_object(
        &mut self,
        shape: impl Hittable + 'static + Send + Sync,
        material: impl Material + 'static + Send + Sync,
    ) {
        self.objects.push(SceneObject {
            shape: Box::new(shape),
            material: Box::new(material),
        });
    }

    pub fn first_hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<(&SceneObject, HitRecord)> {
        let mut closest: Option<(&SceneObject, HitRecord)> = None;
        let mut closest_distance = tmax;
        for object in self.objects.iter() {
            if let Some(hit_record) = object.shape.hit(ray, tmin, closest_distance) {
                closest_distance = hit_record.t;
                closest = Some((object, hit_record));
            }
        }
        closest
    }
}
