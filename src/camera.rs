use crate::geometry::Vec3;
use crate::geometry::Ray;

pub struct Camera {
    pub position: Vec3,
    pub focal_length: f32,
}

impl Camera {
    pub fn ray(&self, x: f32, y: f32) -> Ray {
        Ray {
            origin: self.position,
            direction: Vec3(x, y, self.focal_length),
        }
    }
}
