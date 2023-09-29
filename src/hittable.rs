use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::geometry::dot;

pub struct HitRecord {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm2();
        let b = dot(oc, ray.direction);
        let c = oc.norm2() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t = (-b - discriminant.sqrt()) / a;
        if t < tmin || tmax < t {
            return None;
        }
        let hit_point = ray.at(t);
        let normal = (hit_point - self.center).normalize();
        let front_face = dot(normal, ray.direction) < 0.0;
        Some(HitRecord{ t, hit_point, normal, front_face })
    }
}
