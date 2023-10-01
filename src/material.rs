use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::geometry::dot;
use crate::geometry::random_unit_vector;
use crate::graphics::Color;
use crate::scene::HitRecord;

pub struct Material {
    pub albedo: Color,
    pub polish: f32,
}

fn lambertian(normal: Vec3) -> Vec3 {
    let direction = normal + random_unit_vector();
    if direction.norm2() < 1e-8 {
        // Edge case, avoid scattering in near-zero direction
        normal
    } else {
        direction
    }
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
        let hit_normal = hit_record.normal;
        let diffusion = lambertian(hit_normal);
        let reflection = incoming_ray.direction - 2.0 * dot(incoming_ray.direction, hit_normal) * hit_normal;
        diffusion * (1.0 - self.polish) + reflection * self.polish
    }
}
