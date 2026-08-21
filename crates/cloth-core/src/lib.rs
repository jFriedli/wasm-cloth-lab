mod cloth;
mod material;
mod math;
pub use cloth::{Attachment, Cloth, ClothMetrics, ConstraintKind};
pub use material::{Material, MaterialKind};
pub use math::Vec3;

pub fn relative_airflow(world_wind: Vec3, container_velocity: Vec3) -> Vec3 {
    world_wind - container_velocity
}
pub fn inertial_acceleration(container_acceleration: Vec3) -> Vec3 {
    container_acceleration * -1.
}

#[cfg(test)]
mod mapping_tests {
    use super::*;
    #[test]
    fn browser_right_maps_left() {
        assert_eq!(
            relative_airflow(Vec3::ZERO, Vec3::new(4., 0., 0.)),
            Vec3::new(-4., 0., 0.)
        );
        assert_eq!(
            inertial_acceleration(Vec3::new(5., 0., 0.)),
            Vec3::new(-5., 0., 0.)
        );
    }
}
