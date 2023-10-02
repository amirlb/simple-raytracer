use crate::geometry::Ray;
use crate::geometry::Vec3;
use crate::geometry::dot;
use crate::geometry::random_unit_vector;
use crate::graphics::Color;
use crate::graphics::WHITE;
use crate::scene::HitRecord;
use crate::scene::Material;

pub struct Opaque {
    pub albedo: Color,
    pub polish: f32,
}

fn reflect(vec: Vec3, plane_normal: Vec3) -> Vec3 {
    vec - 2.0 * dot(vec, plane_normal) * plane_normal
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

fn scatter_direction(incoming_ray: Vec3, hit_record: &HitRecord, polish: f32) -> Vec3 {
    let diffusion = lambertian(hit_record.normal);
    let reflection = reflect(incoming_ray, hit_record.normal);
    (1.0 - polish) * diffusion + polish * reflection
}

impl Material for Opaque {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        if dot(ray.direction, hit_record.normal) > 0.0 {
            // ray is coming from inside the body
            return None
        }

        return Some((
            self.albedo,
            Ray {
                origin: hit_record.hit_point,
                direction: scatter_direction(ray.direction.normalize(), hit_record, self.polish),
            },
        ))
    }
}

pub struct Transparent {
    pub refraction_index: f32,
}

fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
    // Use Schlick's approximation for reflectance
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos_theta.abs()).powf(5.0);
}

fn refraction_direction(incoming_ray: Vec3, hit_record: &HitRecord, refraction_index: f32) -> Vec3 {
    let cos_theta = -dot(incoming_ray, hit_record.normal);
    if cos_theta.abs() < 1e-6 {
        // Close to parallel ray, just reflect to avoid numerical issues
        return reflect(incoming_ray, hit_record.normal)
    }

    let refraction_ratio = if cos_theta > 0.0 { 1.0 / refraction_index } else { refraction_index };
    let r2 = refraction_ratio * refraction_ratio;

    let parallel_factor_sq = r2 + (1.0 - r2) / (cos_theta * cos_theta);
    if parallel_factor_sq < 0.0 {
        // Total internal reflection
        return reflect(incoming_ray, hit_record.normal)
    }

    if reflectance(cos_theta, refraction_ratio) > rand::random() {
        // Partial reflection
        return reflect(incoming_ray, hit_record.normal)
    }

    let parallel_coef = (refraction_ratio - parallel_factor_sq.sqrt()) * cos_theta;
    refraction_ratio * incoming_ray + parallel_coef * hit_record.normal
}

impl Material for Transparent {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        Some((
            WHITE,
            Ray {
                origin: hit_record.hit_point,
                direction: refraction_direction(ray.direction.normalize(), hit_record, self.refraction_index),
            }
        ))
    }
}
