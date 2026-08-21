# Benchmark methodology

The benchmark is `cargo run --release -p cloth-core --example solver_bench`. It runs the same Rust core used by WASM without renderer, DOM, or JavaScript cost. It is deterministic and prints CSV. The baseline is preserved by Git tag `research-2026-baseline`.

Each case uses a warm-up, then records individual physics-step wall times with `std::time::Instant`. Reports include mean, median, p95, and p99; median is the primary comparison because unrelated host scheduling produced occasional long tails. Timing is specific to the available Linux x86-64 host and is **not** claimed as browser/WASM performance.

## Scenes and metrics

- **B1 hanging:** top edge pinned, gravity, no wind; settling and strain.
- **B2 wind:** constant 5 and 10 simulation-unit airflow; displacement, deformation, and cost.
- **B3 gust:** bounded impulse followed by decay.
- **B4 window shake:** deterministic idle, movement, stop, reversal, violent shake, and settle sequence.
- **B5 violent stress:** stronger alternating airflow/inertial acceleration; finite-state and attachment checks.
- **B6 resolution:** 30×20, 50×32, 75×48, and 100×64.
- **B7 bending/material:** silk and canvas under identical loading.
- **B8 nylon:** material-control case.

Collected numerical metrics are maximum and RMS structural strain, RMS shear error, bending proxy/error, normalized kinetic energy, center of mass, maximum vertex velocity, pinned error, and estimated owned-buffer bytes. NaN/infinity is a hard failure. The deterministic replay gives candidates identical inputs.

## Limitations

These metrics do not fully measure perceived folds or flutter. The benchmark excludes rendering and WASM boundary cost, and `Instant` can be perturbed by host scheduling. Visual evaluation complements—not replaces—the numerical tests. Real title-bar dragging cannot be automated by ordinary browser test runners and must remain a manual acceptance item.
