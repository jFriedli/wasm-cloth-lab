use crate::{Material, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attachment {
    FullEdge,
    TwoPoint,
    TopEdge,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstraintKind {
    Structural,
    Shear,
    Bend,
}
#[derive(Clone, Copy, Debug)]
struct Constraint {
    a: usize,
    b: usize,
    rest: f32,
    compliance: f32,
    kind: ConstraintKind,
}
#[derive(Clone, Copy, Debug)]
struct Particle {
    p: Vec3,
    prev: Vec3,
    force: Vec3,
    inv_mass: f32,
    pin: Vec3,
}

pub struct Cloth {
    pub width: usize,
    pub height: usize,
    particles: Vec<Particle>,
    constraints: Vec<Constraint>,
    indices: Vec<u32>,
    positions: Vec<f32>,
    normals: Vec<f32>,
    pub material: Material,
    pub attachment: Attachment,
    pub iterations: usize,
    time: f32,
    grab: Option<(usize, Vec3)>,
}
impl Cloth {
    pub fn new(width: usize, height: usize, material: Material, attachment: Attachment) -> Self {
        assert!(width >= 2 && height >= 2);
        let sx = 2.7 / (width - 1) as f32;
        let sy = 1.65 / (height - 1) as f32;
        let mut particles = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let p = Vec3::new(
                    x as f32 * sx,
                    0.825 - y as f32 * sy,
                    0.015 * ((x * 17 + y * 31) % 7) as f32 / 7.,
                );
                particles.push(Particle {
                    p,
                    prev: p,
                    force: Vec3::ZERO,
                    inv_mass: 1. / material.mass,
                    pin: p,
                });
            }
        }
        let mut s = Self {
            width,
            height,
            particles,
            constraints: vec![],
            indices: vec![],
            positions: vec![0.; width * height * 3],
            normals: vec![0.; width * height * 3],
            material,
            attachment,
            iterations: 7,
            time: 0.,
            grab: None,
        };
        s.build_topology();
        s.apply_pins();
        s.update_buffers();
        s
    }
    fn id(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    fn add_constraint(&mut self, a: usize, b: usize, c: f32, k: ConstraintKind) {
        let rest = (self.particles[a].p - self.particles[b].p).len();
        self.constraints.push(Constraint {
            a,
            b,
            rest,
            compliance: c,
            kind: k,
        });
    }
    fn build_topology(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.id(x, y);
                if x + 1 < self.width {
                    self.add_constraint(
                        i,
                        self.id(x + 1, y),
                        self.material.stretch,
                        ConstraintKind::Structural,
                    )
                }
                if y + 1 < self.height {
                    self.add_constraint(
                        i,
                        self.id(x, y + 1),
                        self.material.stretch,
                        ConstraintKind::Structural,
                    )
                }
                if x + 1 < self.width && y + 1 < self.height {
                    self.add_constraint(
                        i,
                        self.id(x + 1, y + 1),
                        self.material.shear,
                        ConstraintKind::Shear,
                    );
                    self.add_constraint(
                        self.id(x + 1, y),
                        self.id(x, y + 1),
                        self.material.shear,
                        ConstraintKind::Shear,
                    );
                }
                if x + 2 < self.width {
                    self.add_constraint(i, self.id(x + 2, y), self.material.bend, ConstraintKind::Bend)
                }
                if y + 2 < self.height {
                    self.add_constraint(i, self.id(x, y + 2), self.material.bend, ConstraintKind::Bend)
                }
                if x + 1 < self.width && y + 1 < self.height {
                    let a = i as u32;
                    let b = self.id(x + 1, y) as u32;
                    let c = self.id(x, y + 1) as u32;
                    let d = self.id(x + 1, y + 1) as u32;
                    self.indices.extend([a, c, b, b, c, d]);
                }
            }
        }
    }
    fn apply_pins(&mut self) {
        for p in &mut self.particles {
            p.inv_mass = 1. / self.material.mass;
        }
        match self.attachment {
            Attachment::FullEdge => {
                for y in 0..self.height {
                    let i = y * self.width;
                    self.particles[i].inv_mass = 0.;
                }
            }
            Attachment::TwoPoint => {
                self.particles[0].inv_mass = 0.;
                let i = (self.height - 1) * self.width;
                self.particles[i].inv_mass = 0.;
            }
            Attachment::TopEdge => {
                for x in 0..self.width {
                    self.particles[x].inv_mass = 0.;
                }
            }
        }
    }
    pub fn set_attachment(&mut self, a: Attachment) {
        self.attachment = a;
        self.apply_pins();
    }
    pub fn set_pin_offset(&mut self, offset: Vec3) {
        for p in &mut self.particles {
            if p.inv_mass == 0. {
                p.p = p.pin + offset;
                p.prev = p.p;
            }
        }
    }
    pub fn set_grab(&mut self, u: f32, v: f32, target: Vec3) {
        let x = (u.clamp(0., 1.) * (self.width - 1) as f32).round() as usize;
        let y = (v.clamp(0., 1.) * (self.height - 1) as f32).round() as usize;
        self.grab = Some((self.id(x, y), target));
    }
    pub fn move_grab(&mut self, target: Vec3) {
        if let Some((_, current)) = &mut self.grab {
            *current = target;
        }
    }
    pub fn clear_grab(&mut self) {
        self.grab = None;
    }
    pub fn step(&mut self, dt: f32, gravity: Vec3, airflow: Vec3, inertia: Vec3) {
        self.time += dt;
        for p in &mut self.particles {
            p.force = Vec3::ZERO;
        }
        self.apply_aerodynamics(airflow);
        let damp = (-self.material.damping * dt).exp();
        for p in &mut self.particles {
            if p.inv_mass > 0. {
                let vel = (p.p - p.prev) * damp;
                let old = p.p;
                let accel = gravity + inertia + p.force * p.inv_mass;
                p.p = p.p + vel + accel * (dt * dt);
                p.prev = old;
            }
        }
        for _ in 0..self.iterations {
            for c in &self.constraints {
                let pa = self.particles[c.a];
                let pb = self.particles[c.b];
                let d = pa.p - pb.p;
                let l = d.len();
                let w = pa.inv_mass + pb.inv_mass;
                if l < 1e-7 || w == 0. {
                    continue;
                }
                let alpha = c.compliance / (dt * dt);
                let dl = -(l - c.rest) / (w + alpha);
                let corr = d * (dl / l);
                self.particles[c.a].p += corr * pa.inv_mass;
                self.particles[c.b].p -= corr * pb.inv_mass;
            }
            if let Some((i, target)) = self.grab {
                self.particles[i].p = target;
            }
        }
        for p in &mut self.particles {
            if p.inv_mass == 0. {
                p.p = p.pin;
                p.prev = p.pin;
            }
            if !p.p.finite() {
                p.p = p.pin;
                p.prev = p.pin;
            }
        }
        self.update_buffers();
    }
    fn apply_aerodynamics(&mut self, air: Vec3) {
        let tris = self.indices.clone();
        for t in tris.as_chunks::<3>().0 {
            let a = t[0] as usize;
            let b = t[1] as usize;
            let c = t[2] as usize;
            let e1 = self.particles[b].p - self.particles[a].p;
            let e2 = self.particles[c].p - self.particles[a].p;
            let area_n = e1.cross(e2) * 0.5;
            let n = area_n.normalized();
            let pressure = air.dot(n);
            let area = area_n.len();
            let normal_force = n * (pressure.abs() * pressure * area * self.material.drag * 0.45);
            let force = normal_force + air * (air.len() * area * self.material.drag * 0.018);
            for i in [a, b, c] {
                self.particles[i].force += force / 3.;
            }
        }
    }
    fn update_buffers(&mut self) {
        self.positions.fill(0.);
        self.normals.fill(0.);
        for (i, p) in self.particles.iter().enumerate() {
            self.positions[i * 3] = p.p.x;
            self.positions[i * 3 + 1] = p.p.y;
            self.positions[i * 3 + 2] = p.p.z;
        }
        for t in self.indices.as_chunks::<3>().0 {
            let a = t[0] as usize;
            let b = t[1] as usize;
            let c = t[2] as usize;
            let n =
                (self.particles[b].p - self.particles[a].p).cross(self.particles[c].p - self.particles[a].p);
            for i in [a, b, c] {
                self.normals[i * 3] += n.x;
                self.normals[i * 3 + 1] += n.y;
                self.normals[i * 3 + 2] += n.z;
            }
        }
        for n in self.normals.as_chunks_mut::<3>().0 {
            let v = Vec3::new(n[0], n[1], n[2]).normalized();
            n.copy_from_slice(&[v.x, v.y, v.z]);
        }
    }
    pub fn positions(&self) -> &[f32] {
        &self.positions
    }
    pub fn normals(&self) -> &[f32] {
        &self.normals
    }
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }
    pub fn pinned_count(&self) -> usize {
        self.particles.iter().filter(|p| p.inv_mass == 0.).count()
    }
    pub fn average_x(&self) -> f32 {
        self.particles.iter().map(|p| p.p.x).sum::<f32>() / self.particles.len() as f32
    }
    pub fn max_edge_error(&self) -> f32 {
        self.constraints
            .iter()
            .filter(|c| c.kind == ConstraintKind::Structural)
            .map(|c| ((self.particles[c.a].p - self.particles[c.b].p).len() - c.rest).abs() / c.rest)
            .fold(0., f32::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cloth() -> Cloth {
        Cloth::new(12, 8, Material::default(), Attachment::FullEdge)
    }
    #[test]
    fn topology_and_pins() {
        let c = cloth();
        assert_eq!(c.pinned_count(), 8);
        assert_eq!(c.indices.len() / 3, 154);
        assert!(c.constraint_count() > 200)
    }
    #[test]
    fn gravity_sags_but_pins_hold() {
        let mut c = cloth();
        let pin = c.positions()[1];
        for _ in 0..240 {
            c.step(1. / 120., Vec3::new(0., -9.81, 0.), Vec3::ZERO, Vec3::ZERO)
        }
        assert_eq!(c.positions()[1], pin);
        assert!(c.positions()[(7 * 12 + 11) * 3 + 1] < -0.7);
        assert!(c.max_edge_error() < 0.15)
    }
    #[test]
    fn wind_sign() {
        let mut r = cloth();
        let start = r.average_x();
        for _ in 0..180 {
            r.step(1. / 120., Vec3::ZERO, Vec3::new(-8., 0., 2.), Vec3::ZERO)
        }
        assert!(r.average_x() < start)
    }
    #[test]
    fn inertia_sign() {
        let mut r = cloth();
        let start = r.average_x();
        for _ in 0..60 {
            r.step(1. / 120., Vec3::ZERO, Vec3::ZERO, Vec3::new(-15., 0., 0.))
        }
        assert!(r.average_x() < start)
    }
    #[test]
    fn deterministic() {
        let mut a = cloth();
        let mut b = cloth();
        for _ in 0..10 {
            a.step(
                1. / 120.,
                Vec3::new(0., -9.81, 0.),
                Vec3::new(2., 0., 1.),
                Vec3::ZERO,
            );
            b.step(
                1. / 120.,
                Vec3::new(0., -9.81, 0.),
                Vec3::new(2., 0., 1.),
                Vec3::ZERO,
            )
        }
        assert_eq!(a.positions(), b.positions())
    }
    #[test]
    fn extreme_alternating_window_input_stays_finite_and_attached() {
        let mut c = cloth();
        let pinned = c.positions()[..3].to_vec();
        for frame in 0..720 {
            let sign = if (frame / 12) % 2 == 0 { 1. } else { -1. };
            c.step(
                1. / 120.,
                Vec3::new(0., -9.81, 0.),
                Vec3::new(52. * sign, 0., 3.),
                Vec3::new(57. * sign, 0., 0.),
            );
        }
        assert!(c.positions().iter().all(|value| value.is_finite()));
        assert_eq!(&c.positions()[..3], pinned.as_slice());
        assert!(c.max_edge_error() < 0.35);
    }
    #[test]
    fn material_properties() {
        let s = Material::preset(crate::MaterialKind::Silk);
        let c = Material::preset(crate::MaterialKind::Canvas);
        assert!(s.mass < c.mass && s.bend > c.bend)
    }
}
