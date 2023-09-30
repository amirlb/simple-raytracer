use crate::geometry::Ray;
use crate::geometry::dot;
use crate::geometry::random_unit_vector;
use crate::graphics::Color;
use crate::scene::HitRecord;

pub struct Material {}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        if dot(hit_record.normal, ray.direction) > 0.0 {
            // ray is coming from inside the body
            return None
        }

        let diffused_ray = Ray {
            origin: hit_record.hit_point,
            direction: hit_record.normal + random_unit_vector(),
        };
        return Some((
            Color{ red: 0.5, green: 0.5, blue: 0.5 },
            diffused_ray,
        ))
    }
}
