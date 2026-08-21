use cloth_core::{Attachment, Cloth, Material, MaterialKind, Vec3};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    cloth: Cloth,
}
fn material(id: u8) -> Material {
    Material::preset(match id {
        0 => MaterialKind::Silk,
        2 => MaterialKind::Canvas,
        3 => MaterialKind::Nylon,
        4 => MaterialKind::Rubber,
        _ => MaterialKind::Cotton,
    })
}
#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(w: usize, h: usize, kind: u8) -> Self {
        Self {
            cloth: Cloth::new(w, h, material(kind), Attachment::FullEdge),
        }
    }
    #[allow(clippy::too_many_arguments)] // Flat scalars keep the JS/WASM boundary allocation-free.
    pub fn step(&mut self, dt: f32, g: f32, wx: f32, wy: f32, wz: f32, ax: f32, ay: f32, az: f32) {
        self.cloth.step(
            dt,
            Vec3::new(0., -g, 0.),
            Vec3::new(wx, wy, wz),
            Vec3::new(-ax, -ay, -az),
        );
    }
    pub fn positions_ptr(&self) -> *const f32 {
        self.cloth.positions().as_ptr()
    }
    pub fn normals_ptr(&self) -> *const f32 {
        self.cloth.normals().as_ptr()
    }
    pub fn indices_ptr(&self) -> *const u32 {
        self.cloth.indices().as_ptr()
    }
    pub fn vertex_count(&self) -> usize {
        self.cloth.positions().len() / 3
    }
    pub fn index_count(&self) -> usize {
        self.cloth.indices().len()
    }
    pub fn constraint_count(&self) -> usize {
        self.cloth.constraint_count()
    }
    pub fn set_attachment(&mut self, id: u8) {
        self.cloth.set_attachment(match id {
            1 => Attachment::TwoPoint,
            2 => Attachment::TopEdge,
            _ => Attachment::FullEdge,
        });
    }
    pub fn set_grab(&mut self, u: f32, v: f32, x: f32, y: f32, z: f32) {
        self.cloth.set_grab(u, v, Vec3::new(x, y, z));
    }
    pub fn move_grab(&mut self, x: f32, y: f32, z: f32) {
        self.cloth.move_grab(Vec3::new(x, y, z));
    }
    pub fn clear_grab(&mut self) {
        self.cloth.clear_grab();
    }
}
