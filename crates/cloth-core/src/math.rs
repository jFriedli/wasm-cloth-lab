#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn dot(self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    pub fn cross(self, b: Self) -> Self {
        Self::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }
    pub fn len2(self) -> f32 {
        self.dot(self)
    }
    pub fn len(self) -> f32 {
        self.len2().sqrt()
    }
    pub fn normalized(self) -> Self {
        let l = self.len();
        if l > 1e-7 { self / l } else { Self::ZERO }
    }
    pub fn finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}
impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self::new(self.x + b.x, self.y + b.y, self.z + b.z)
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, s: f32) -> Self {
        self * (1.0 / s)
    }
}
impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, b: Self) {
        *self = *self + b
    }
}
impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, b: Self) {
        *self = *self - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vectors() {
        assert_eq!(
            Vec3::new(1., 0., 0.).cross(Vec3::new(0., 1., 0.)),
            Vec3::new(0., 0., 1.)
        );
        assert!((Vec3::new(3., 4., 0.).len() - 5.).abs() < 1e-6);
    }
}
