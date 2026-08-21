# Production decision

## Original solver

The preserved production commit uses a serial Gauss–Seidel compliant position solver: structural and shear distances, two-edge distance bending, Verlet integration, exponential damping, triangle pressure/skin drag, 1/120 s frontend fixed steps, and seven iterations. It is commonly described in the project as XPBD-style, but it lacks XPBD’s accumulated multiplier term.

## Candidates researched and implemented

Full XPBD multipliers, 7×1 Small Steps XPBD, 2×4 hybrid XPBD, cotangent quadratic isometric bending, and surface-relative triangle aerodynamics were implemented behind core benchmark configuration. Projective Dynamics, discrete-shell FEM, IPC/C-IPC, OGC, WebGPU compute, and self-collision pipelines were evaluated from primary literature but not reduced to misleading partial implementations.

## Quantitative result

In B4 at 50×32, baseline/relative-aero medians were 1.042/1.088 ms and maximum structural strain 0.657/0.624. Hybrid XPBD reduced maximum strain to 0.560 but raised median cost to 1.312 ms and reduced final normalized kinetic energy from 4.285 to 0.995. Small Steps reduced maximum strain to 0.491 at 50×32, then diverged at 75×48 and catastrophically failed at 100×64. Isometric bending cost 3.375 ms and increased uncalibrated maximum strain to 1.196. See [BENCHMARK_RESULTS.md](BENCHMARK_RESULTS.md) for the matrix and limitations.

## Final architecture

Production remains the stable serial compliant Gauss–Seidel solver with the existing distance constraint network, fixed step, materials, and interaction gains. It now uses **triangle surface-relative aerodynamic pressure and skin drag** and no longer clones topology every physics step. Experimental solver modes are benchmark-only Rust core paths and do not alter the WASM API or normal UI.

This is intentionally conservative. It preserves the window-driven feel and Ultra stability while correcting the clearest physical omission. The code and docs now accurately distinguish compliant PBD from full XPBD.

## Rejected approaches

- Full XPBD was not promoted because existing artistic compliance values do not transfer unchanged and the tested configuration increased strain.
- Direct Small Steps was rejected because it exploded at production High/Ultra resolutions.
- Hybrid XPBD was rejected because its moderate stretch improvement cost time and removed much of the lively transient energy.
- Isometric bending was rejected from production because direct scalar reuse was slower and worse; it needs resolution-aware energy calibration before it is a visual improvement.
- Projective Dynamics was not prototyped incompletely: a fair solver requires sparse global infrastructure and equivalent energies, with little expected benefit at this size.
- Full IPC/C-IPC was rejected because nonlinear optimization, CCD, barriers, and sparse Hessians solve a contact-guarantee problem outside this low-latency flag’s priorities.
- OGC was not claimed or adapted; its strongest advantages target parallel contact-heavy workloads.
- Production self-collision was rejected because a top-edge flag has limited benefit and variable collision cost threatens shake latency.
- WebGPU physics was deferred because current CPU scale is adequate and migration would add synchronization, compatibility, and duplicate-solver cost.

## Next research step

The next visible improvement most likely to pay off is a **resolution-normalized triangle strain model with warp/weft/shear parameters**, paired with a calibrated dihedral or isometric bending energy. It should first be tested at all four resolutions and evaluated with deterministic visual captures. If performance constraints were removed, a nonlinear discrete-shell model with C-IPC contact would be the preferred high-fidelity direction because it combines constitutive shell energy with robust finite-thickness contact—not because it is newer, but because contact guarantees and calibrated material response would then outweigh latency and complexity.
