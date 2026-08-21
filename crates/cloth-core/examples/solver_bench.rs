use cloth_core::{Attachment, Cloth, ClothMetrics, Material, MaterialKind, Vec3};
use std::{hint::black_box, time::Instant};

#[derive(Clone, Copy)]
enum Scene {
    Hanging,
    Wind5,
    Wind10,
    Gust,
    Shake,
    Violent,
}

fn inputs(scene: Scene, frame: usize) -> (Vec3, Vec3, Vec3) {
    let gravity = Vec3::new(0., -9.81, 0.);
    match scene {
        Scene::Hanging => (gravity, Vec3::ZERO, Vec3::ZERO),
        Scene::Wind5 => (gravity, Vec3::new(5., 0., 1.), Vec3::ZERO),
        Scene::Wind10 => (gravity, Vec3::new(10., 0., 2.), Vec3::ZERO),
        Scene::Gust => (
            gravity,
            if frame < 60 {
                Vec3::new(18., 0., 4.)
            } else {
                Vec3::ZERO
            },
            Vec3::ZERO,
        ),
        Scene::Shake => {
            let phase = frame % 120;
            let sign = if phase < 30 || (60..90).contains(&phase) {
                1.
            } else {
                -1.
            };
            (
                gravity,
                Vec3::new(18. * sign, 0., 2.),
                Vec3::new(22. * sign, 0., 0.),
            )
        }
        Scene::Violent => {
            let sign = if (frame / 8).is_multiple_of(2) { 1. } else { -1. };
            (
                gravity,
                Vec3::new(52. * sign, 0., 3.),
                Vec3::new(57. * sign, 0., 0.),
            )
        }
    }
}

fn percentile(samples: &mut [f64], p: f64) -> f64 {
    samples.sort_by(f64::total_cmp);
    samples[((samples.len() - 1) as f64 * p).round() as usize]
}

fn run(name: &str, w: usize, h: usize, scene: Scene, material: MaterialKind) {
    let dt = 1. / 120.;
    let mut cloth = Cloth::new(w, h, Material::preset(material), Attachment::TopEdge);
    for frame in 0..60 {
        let (g, wind, inertia) = inputs(scene, frame);
        cloth.step(dt, g, wind, inertia);
    }
    let mut timings = Vec::with_capacity(360);
    let mut max_strain = 0_f32;
    let mut final_metrics = ClothMetrics::default();
    for frame in 60..420 {
        let (g, wind, inertia) = inputs(scene, frame);
        let start = Instant::now();
        cloth.step(dt, g, wind, inertia);
        timings.push(start.elapsed().as_secs_f64() * 1000.);
        final_metrics = cloth.metrics(dt);
        max_strain = max_strain.max(final_metrics.max_structural_strain);
        black_box(cloth.positions());
    }
    let mean = timings.iter().sum::<f64>() / timings.len() as f64;
    let median = percentile(&mut timings.clone(), 0.5);
    let p95 = percentile(&mut timings.clone(), 0.95);
    let p99 = percentile(&mut timings, 0.99);
    println!(
        "{name},{w}x{h},{},{mean:.6},{median:.6},{p95:.6},{p99:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.3},{}",
        w * h,
        max_strain,
        final_metrics.rms_structural_strain,
        final_metrics.rms_shear_strain,
        final_metrics.rms_bend_error,
        final_metrics.kinetic_energy,
        final_metrics.max_velocity,
        cloth.estimated_bytes()
    );
}

fn main() {
    println!(
        "scene,mesh,vertices,mean_ms,median_ms,p95_ms,p99_ms,max_structural_strain,final_rms_structural,final_rms_shear,final_rms_bend,final_energy,max_velocity,estimated_bytes"
    );
    run("B1_hanging", 50, 32, Scene::Hanging, MaterialKind::Cotton);
    run("B2_wind5", 50, 32, Scene::Wind5, MaterialKind::Cotton);
    run("B2_wind10", 50, 32, Scene::Wind10, MaterialKind::Cotton);
    run("B3_gust", 50, 32, Scene::Gust, MaterialKind::Cotton);
    run("B4_shake", 50, 32, Scene::Shake, MaterialKind::Cotton);
    run("B5_violent", 50, 32, Scene::Violent, MaterialKind::Cotton);
    for (w, h) in [(30, 20), (50, 32), (75, 48), (100, 64)] {
        run("B6_scaling", w, h, Scene::Shake, MaterialKind::Cotton);
    }
    run("B7_bend_silk", 50, 32, Scene::Gust, MaterialKind::Silk);
    run("B7_bend_canvas", 50, 32, Scene::Gust, MaterialKind::Canvas);
    run("B8_nylon", 50, 32, Scene::Wind10, MaterialKind::Nylon);
}
