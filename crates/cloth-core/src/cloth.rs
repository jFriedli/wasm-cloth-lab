use std::collections::HashMap;

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
#[derive(Clone, Copy, Debug, Default)]
pub struct ClothMetrics {
    pub max_structural_strain: f32,
    pub rms_structural_strain: f32,
    pub rms_shear_strain: f32,
    pub rms_bend_error: f32,
    pub pinned_error: f32,
    pub kinetic_energy: f32,
    pub center_of_mass: Vec3,
    pub max_velocity: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolverMode {
    Baseline,
    Xpbd,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AerodynamicsMode {
    Baseline,
    RelativeVelocity,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BendingMode {
    LongRangeDistance,
    Isometric,
}
#[derive(Clone, Copy, Debug)]
pub struct SolverConfig {
    pub mode: SolverMode,
    pub aerodynamics: AerodynamicsMode,
    pub bending: BendingMode,
    pub substeps: usize,
    pub iterations: usize,
}
impl SolverConfig {
    pub const fn baseline(iterations: usize) -> Self {
        Self {
            mode: SolverMode::Baseline,
            aerodynamics: AerodynamicsMode::Baseline,
            bending: BendingMode::LongRangeDistance,
            substeps: 1,
            iterations,
        }
    }
    pub const fn xpbd(iterations: usize) -> Self {
        Self {
            mode: SolverMode::Xpbd,
            aerodynamics: AerodynamicsMode::Baseline,
            bending: BendingMode::LongRangeDistance,
            substeps: 1,
            iterations,
        }
    }
    pub const fn small_steps(substeps: usize) -> Self {
        Self {
            mode: SolverMode::Xpbd,
            aerodynamics: AerodynamicsMode::Baseline,
            bending: BendingMode::LongRangeDistance,
            substeps,
            iterations: 1,
        }
    }
    pub const fn hybrid(substeps: usize, iterations: usize) -> Self {
        Self {
            mode: SolverMode::Xpbd,
            aerodynamics: AerodynamicsMode::Baseline,
            bending: BendingMode::LongRangeDistance,
            substeps,
            iterations,
        }
    }
    pub const fn with_relative_aerodynamics(mut self) -> Self {
        self.aerodynamics = AerodynamicsMode::RelativeVelocity;
        self
    }
    pub const fn with_isometric_bending(mut self) -> Self {
        self.bending = BendingMode::Isometric;
        self
    }
}
#[derive(Clone, Copy, Debug)]
struct Constraint {
    a: usize,
    b: usize,
    rest: f32,
    compliance: f32,
    kind: ConstraintKind,
    lambda: f32,
}
#[derive(Clone, Copy, Debug)]
struct Particle {
    p: Vec3,
    prev: Vec3,
    force: Vec3,
    inv_mass: f32,
    pin: Vec3,
}
#[derive(Clone, Copy, Debug)]
struct IsometricBend {
    ids: [usize; 4],
    q: [f32; 16],
    lambda: f32,
}

pub struct Cloth {
    pub width: usize,
    pub height: usize,
    particles: Vec<Particle>,
    constraints: Vec<Constraint>,
    isometric_bends: Vec<IsometricBend>,
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
            isometric_bends: vec![],
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
            lambda: 0.,
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
    fn build_isometric_bends(&mut self) {
        let mut edges: HashMap<(usize, usize), usize> = HashMap::new();
        for tri in self.indices.as_chunks::<3>().0 {
            let ids = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
            for &(a, b, opposite) in &[
                (ids[0], ids[1], ids[2]),
                (ids[1], ids[2], ids[0]),
                (ids[2], ids[0], ids[1]),
            ] {
                let edge = if a < b { (a, b) } else { (b, a) };
                if let Some(other) = edges.remove(&edge) {
                    if let Some(bend) = self.make_isometric_bend(edge.0, edge.1, other, opposite) {
                        self.isometric_bends.push(bend);
                    }
                } else {
                    edges.insert(edge, opposite);
                }
            }
        }
    }
    fn make_isometric_bend(&self, a: usize, b: usize, c: usize, d: usize) -> Option<IsometricBend> {
        let p = [
            self.particles[a].p,
            self.particles[b].p,
            self.particles[c].p,
            self.particles[d].p,
        ];
        let cot = |u: Vec3, v: Vec3| u.dot(v) / u.cross(v).len().max(1e-8);
        let cot_ac = cot(p[1] - p[0], p[2] - p[0]);
        let cot_bc = cot(p[0] - p[1], p[2] - p[1]);
        let cot_ad = cot(p[1] - p[0], p[3] - p[0]);
        let cot_bd = cot(p[0] - p[1], p[3] - p[1]);
        let area = 0.5 * ((p[1] - p[0]).cross(p[2] - p[0]).len() + (p[1] - p[0]).cross(p[3] - p[0]).len());
        if area < 1e-8 {
            return None;
        }
        let k = [
            cot_bc + cot_bd,
            cot_ac + cot_ad,
            -cot_ac - cot_bc,
            -cot_ad - cot_bd,
        ];
        let scale = 3. / area;
        let mut q = [0.; 16];
        for i in 0..4 {
            for j in 0..4 {
                q[i * 4 + j] = scale * k[i] * k[j];
            }
        }
        Some(IsometricBend {
            ids: [a, b, c, d],
            q,
            lambda: 0.,
        })
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
        self.step_with_config(
            dt,
            gravity,
            airflow,
            inertia,
            SolverConfig::baseline(self.iterations).with_relative_aerodynamics(),
        );
    }
    pub fn step_with_config(
        &mut self,
        dt: f32,
        gravity: Vec3,
        airflow: Vec3,
        inertia: Vec3,
        config: SolverConfig,
    ) {
        self.time += dt;
        if config.bending == BendingMode::Isometric && self.isometric_bends.is_empty() {
            self.build_isometric_bends();
        }
        let substeps = config.substeps.max(1);
        let h = dt / substeps as f32;
        for _ in 0..substeps {
            self.substep(h, gravity, airflow, inertia, config);
        }
        self.update_buffers();
    }
    fn substep(&mut self, dt: f32, gravity: Vec3, airflow: Vec3, inertia: Vec3, config: SolverConfig) {
        for p in &mut self.particles {
            p.force = Vec3::ZERO;
        }
        self.apply_aerodynamics(airflow, dt, config.aerodynamics);
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
        if config.mode == SolverMode::Xpbd {
            for c in &mut self.constraints {
                c.lambda = 0.;
            }
            for b in &mut self.isometric_bends {
                b.lambda = 0.;
            }
        }
        for _ in 0..config.iterations.max(1) {
            for c in &mut self.constraints {
                if config.bending == BendingMode::Isometric && c.kind == ConstraintKind::Bend {
                    continue;
                }
                let pa = self.particles[c.a];
                let pb = self.particles[c.b];
                let d = pa.p - pb.p;
                let l = d.len();
                let w = pa.inv_mass + pb.inv_mass;
                if l < 1e-7 || w == 0. {
                    continue;
                }
                let alpha = c.compliance / (dt * dt);
                let constraint = l - c.rest;
                let dl = match config.mode {
                    SolverMode::Baseline => -constraint / (w + alpha),
                    SolverMode::Xpbd => -(constraint + alpha * c.lambda) / (w + alpha),
                };
                c.lambda += dl;
                let corr = d * (dl / l);
                self.particles[c.a].p += corr * pa.inv_mass;
                self.particles[c.b].p -= corr * pb.inv_mass;
            }
            if config.bending == BendingMode::Isometric {
                Self::solve_isometric_bends(
                    &mut self.particles,
                    &mut self.isometric_bends,
                    self.material.bend,
                    dt,
                    config.mode,
                );
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
    }
    fn solve_isometric_bends(
        particles: &mut [Particle],
        bends: &mut [IsometricBend],
        compliance: f32,
        dt: f32,
        mode: SolverMode,
    ) {
        let alpha = compliance / (dt * dt);
        for bend in bends {
            let x = bend.ids.map(|id| particles[id].p);
            let mut gradients = [Vec3::ZERO; 4];
            let mut energy = 0.;
            for i in 0..4 {
                for j in 0..4 {
                    gradients[i] += x[j] * bend.q[i * 4 + j];
                    energy += 0.5 * bend.q[i * 4 + j] * x[i].dot(x[j]);
                }
            }
            let denom = bend
                .ids
                .iter()
                .enumerate()
                .map(|(i, &id)| particles[id].inv_mass * gradients[i].len2())
                .sum::<f32>();
            if denom < 1e-10 {
                continue;
            }
            let dl = match mode {
                SolverMode::Baseline => -energy / (denom + alpha),
                SolverMode::Xpbd => -(energy + alpha * bend.lambda) / (denom + alpha),
            };
            bend.lambda += dl;
            for (i, &id) in bend.ids.iter().enumerate() {
                particles[id].p += gradients[i] * (particles[id].inv_mass * dl);
            }
        }
    }
    fn apply_aerodynamics(&mut self, air: Vec3, dt: f32, mode: AerodynamicsMode) {
        let particles = &mut self.particles;
        for t in self.indices.as_chunks::<3>().0 {
            let a = t[0] as usize;
            let b = t[1] as usize;
            let c = t[2] as usize;
            let e1 = particles[b].p - particles[a].p;
            let e2 = particles[c].p - particles[a].p;
            let area_n = e1.cross(e2) * 0.5;
            let n = area_n.normalized();
            let relative_air = match mode {
                AerodynamicsMode::Baseline => air,
                AerodynamicsMode::RelativeVelocity => {
                    let va = (particles[a].p - particles[a].prev) / dt;
                    let vb = (particles[b].p - particles[b].prev) / dt;
                    let vc = (particles[c].p - particles[c].prev) / dt;
                    air - (va + vb + vc) / 3.
                }
            };
            let pressure = relative_air.dot(n);
            let area = area_n.len();
            let normal_force = n * (pressure.abs() * pressure * area * self.material.drag * 0.45);
            let force =
                normal_force + relative_air * (relative_air.len() * area * self.material.drag * 0.018);
            for i in [a, b, c] {
                particles[i].force += force / 3.;
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
    pub fn metrics(&self, dt: f32) -> ClothMetrics {
        let mut result = ClothMetrics::default();
        let mut structural_sq = 0.;
        let mut shear_sq = 0.;
        let mut bend_sq = 0.;
        let mut structural_n = 0;
        let mut shear_n = 0;
        let mut bend_n = 0;
        for c in &self.constraints {
            let strain = ((self.particles[c.a].p - self.particles[c.b].p).len() - c.rest) / c.rest;
            match c.kind {
                ConstraintKind::Structural => {
                    result.max_structural_strain = result.max_structural_strain.max(strain.abs());
                    structural_sq += strain * strain;
                    structural_n += 1;
                }
                ConstraintKind::Shear => {
                    shear_sq += strain * strain;
                    shear_n += 1;
                }
                ConstraintKind::Bend => {
                    bend_sq += strain * strain;
                    bend_n += 1;
                }
            }
        }
        result.rms_structural_strain = (structural_sq / structural_n.max(1) as f32).sqrt();
        result.rms_shear_strain = (shear_sq / shear_n.max(1) as f32).sqrt();
        result.rms_bend_error = (bend_sq / bend_n.max(1) as f32).sqrt();
        let mut movable_mass = 0.;
        for p in &self.particles {
            result.center_of_mass += p.p;
            if p.inv_mass == 0. {
                result.pinned_error = result.pinned_error.max((p.p - p.pin).len());
            } else {
                let mass = 1. / p.inv_mass;
                let speed = (p.p - p.prev).len() / dt;
                result.kinetic_energy += 0.5 * mass * speed * speed;
                result.max_velocity = result.max_velocity.max(speed);
                movable_mass += mass;
            }
        }
        result.center_of_mass = result.center_of_mass / self.particles.len() as f32;
        if movable_mass > 0. {
            result.kinetic_energy /= movable_mass;
        }
        result
    }
    pub fn estimated_bytes(&self) -> usize {
        self.particles.capacity() * std::mem::size_of::<Particle>()
            + self.constraints.capacity() * std::mem::size_of::<Constraint>()
            + self.isometric_bends.capacity() * std::mem::size_of::<IsometricBend>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + (self.positions.capacity() + self.normals.capacity()) * std::mem::size_of::<f32>()
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
    fn relative_aerodynamics_vanishes_when_cloth_matches_air_velocity() {
        let mut c = cloth();
        let dt = 1. / 120.;
        let air = Vec3::new(4., -1., 2.);
        for p in &mut c.particles {
            p.prev = p.p - air * dt;
            p.force = Vec3::ZERO;
        }
        c.apply_aerodynamics(air, dt, AerodynamicsMode::RelativeVelocity);
        let total = c.particles.iter().fold(Vec3::ZERO, |sum, p| sum + p.force);
        assert!(total.len() < 1e-4, "co-moving air force was {total:?}");
    }
    #[test]
    fn material_properties() {
        let s = Material::preset(crate::MaterialKind::Silk);
        let c = Material::preset(crate::MaterialKind::Canvas);
        assert!(s.mass < c.mass && s.bend > c.bend)
    }
}
