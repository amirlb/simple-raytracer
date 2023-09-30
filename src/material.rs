use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::geometry::dot;
use crate::geometry::random_unit_vector;
use crate::graphics::Color;
use crate::scene::HitRecord;

pub struct Material {
    pub albedo: Color,
    pub reflective: bool,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        if dot(hit_record.normal, ray.direction) > 0.0 {
            // ray is coming from inside the body
            return None
        }

        let direction = self.scatter_direction(ray, hit_record);
        return Some((
            self.albedo,
            Ray { origin: hit_record.hit_point, direction },
        ))
    }

    fn scatter_direction(&self, incoming_ray: &Ray, hit_record: &HitRecord) -> Vec3 {
        if self.reflective {
            incoming_ray.direction - 2.0 * dot(incoming_ray.direction, hit_record.normal) * hit_record.normal
        } else {
            let lambertian = hit_record.normal + random_unit_vector();
            if lambertian.norm2() < 1e-8 {
                // Edge case, avoid scattering in near-zero direction
                hit_record.normal
            } else {
                lambertian
            }
        }
    }
}
