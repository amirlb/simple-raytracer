use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::geometry::dot;
use crate::scene::HitRecord;
use crate::scene::Hittable;

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
        let mut t = (-b - discriminant.sqrt()) / a;
        if t <= tmin {
            t = (-b + discriminant.sqrt()) / a;
        }
        if t <= tmin || tmax <= t {
            return None;
        }
        let hit_point = ray.at(t);
        let normal = (hit_point - self.center) / self.radius;
        Some(HitRecord{ t, hit_point, normal })
    }

    fn contains(&self, point: Vec3) -> bool {
        (point - self.center).norm2() < self.radius * self.radius
    }
}

pub struct Medium {
    pub shape: Box<dyn Hittable + 'static + Send + Sync>,
    pub density: f32,
}

impl Hittable for Medium {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let entry_time = if self.shape.contains(ray.at(tmin)) {
            tmin
        } else {
            match self.shape.hit(ray, tmin, tmax) {
                None => { return None }
                Some(entry) => { entry.t }
            }
        };
        let exit_time = match self.shape.hit(ray, entry_time, tmax) {
            None => tmax,
            Some(exit) => exit.t,
        };
        let free_path = -(1.0 - rand::random::<f32>()).ln() / self.density;
        let t = entry_time + free_path;
        if entry_time + free_path < exit_time - entry_time {
            Some(HitRecord {
                t,
                hit_point: ray.at(t),
                normal: ray.direction,
            })
        } else {
            None
        }
    }

    fn contains(&self, _: Vec3) -> bool {
        false
    }
}
