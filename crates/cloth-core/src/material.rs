#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialKind {
    Silk,
    Cotton,
    Canvas,
    Nylon,
    Rubber,
}
#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub mass: f32,
    pub stretch: f32,
    pub shear: f32,
    pub bend: f32,
    pub damping: f32,
    pub drag: f32,
}
impl Material {
    pub fn preset(kind: MaterialKind) -> Self {
        match kind {
            MaterialKind::Silk => Self {
                mass: 0.55,
                stretch: 2e-7,
                shear: 7e-7,
                bend: 3e-4,
                damping: 0.7,
                drag: 1.35,
            },
            MaterialKind::Cotton => Self {
                mass: 1.,
                stretch: 8e-8,
                shear: 3e-7,
                bend: 8e-5,
                damping: 1.2,
                drag: 1.05,
            },
            MaterialKind::Canvas => Self {
                mass: 2.5,
                stretch: 2e-8,
                shear: 8e-8,
                bend: 1e-5,
                damping: 2.,
                drag: 0.8,
            },
            MaterialKind::Nylon => Self {
                mass: 0.7,
                stretch: 5e-8,
                shear: 2e-7,
                bend: 1.5e-4,
                damping: 0.65,
                drag: 1.25,
            },
            MaterialKind::Rubber => Self {
                mass: 1.6,
                stretch: 8e-5,
                shear: 1e-4,
                bend: 3e-5,
                damping: 3.2,
                drag: 1.0,
            },
        }
    }
}
impl Default for Material {
    fn default() -> Self {
        Self::preset(MaterialKind::Cotton)
    }
}
