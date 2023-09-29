#[derive(Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, x: f32) -> Vec3 {
        Vec3(self.0 * x, self.1 * x, self.2 * x)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 { vec * self }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, x: f32) -> Vec3 { self * (1.0 / x) }
}

pub fn dot(vec1: Vec3, vec2: Vec3) -> f32 {
    vec1.0 * vec2.0 + vec1.1 * vec2.1 + vec1.2 * vec2.2
}

impl Vec3 {
    pub fn norm2(self) -> f32 {
        dot(self, self)
    }

    pub fn norm(self) -> f32 {
        self.norm2().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self / self.norm()
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
