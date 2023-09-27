#[derive(Clone)]
#[derive(Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn norm(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let factor = 1.0 / self.norm();
        Vec3(self.0 * factor, self.1 * factor, self.2 * factor)
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
