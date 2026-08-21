# Cloth simulation review (2026)

## Executive summary

For this application, a serial CPU constraint solver remains the best fit. The mesh is modest, inputs must affect the next fixed step, deterministic Rust tests are valuable, and WebGL2/ordinary GitHub Pages compatibility is intentional. Modern offline contact solvers solve a harder problem with orders of magnitude more machinery. WebGPU is attractive for a future high-resolution/contact mode, but a migration would add a second physics architecture and Jacobi/graph-coloring concerns without improving the defining browser-window interaction at the current mesh sizes.

The most relevant improvements are narrower: audit the compliance equation, compare Small Steps, replace the stretch-coupled long-range bending proxy experimentally, and correct aerodynamics to use air velocity relative to the moving cloth surface. Benchmarks ultimately favored retaining the stable serial compliant-projection solver while adopting surface-relative aerodynamics and eliminating a per-step topology allocation. True XPBD, Small Steps, hybrid substeps, and isometric bending remain research implementations rather than production defaults.

## Existing architecture at the preserved baseline

At tag `research-2026-baseline` (`a475348dc9dc54b9c97c8f000e391c135806dd6a`), `cloth-core` stores particles as an array of structs with current/previous position, force, inverse mass, and pin target. A regular triangulated grid creates horizontal/vertical distance constraints, both cell diagonals for shear, and two-edge horizontal/vertical distance constraints as a bending proxy. Constraints are projected serially in Gauss–Seidel order. Verlet-style integration uses exponential velocity damping. Pins have zero inverse mass.

The denominator includes `compliance / dt²`, but each projection uses `-C/(w+alpha)` and does not accumulate the constraint multiplier. It is therefore **compliant PBD / XPBD-like**, not the full XPBD update from Macklin et al. The frontend accumulator calls 1/120 s fixed steps, capped after long frames. Default Ultra is 100×64; the historic nominal/default mesh documented during earlier development was 50×32. Seven projection sweeps are used.

Aerodynamics are triangle based: area-weighted normal pressure plus a smaller tangential drag term, distributed to vertices. At baseline, those forces use the air vector but not triangle velocity. Browser velocity and acceleration are correctly separate: window velocity is negated into relative airflow, while window acceleration is negated into pseudo-acceleration.

## Solver families

### Mass–spring and PBD

Explicit mass–spring systems are easy to implement but stiff cloth demands small steps or implicit solves. Classical PBD directly projects geometric constraints and is robust and interactive, but effective stiffness varies with step size and iteration count. Serial Gauss–Seidel converges quickly and immediately propagates corrections, but is order dependent and difficult to parallelize. Jacobi is parallel but generally needs more iterations; graph coloring permits parallel independent Gauss–Seidel batches at preprocessing and scheduling cost. See Müller et al. (2007) and the Bender–Müller–Macklin survey (2017).

### XPBD and Small Steps

XPBD introduces compliance and a total Lagrange multiplier. For scalar constraint `C`, gradient-weighted inverse mass `w`, and `alpha = compliance / h²`, the increment is:

`delta_lambda = (-C - alpha * lambda) / (w + alpha)`.

The correction is `M^-1 grad(C) delta_lambda`. Resetting total multipliers at the start of a physical substep and accumulating them across iterations makes compliance substantially less dependent on iteration count and step size. Warm starting across frames is not automatically valid: it changes the method unless multipliers and external loading are handled consistently.

Small Steps (Macklin et al., 2019) observes that many small substeps with one projection can outperform one large step with many projections because nonlinear constraints are re-linearized and velocities updated more often. The claim is not a universal configuration recipe. In this grid, directly replacing 1×7 with 7×1 improved default-mesh strain but destabilized the finer meshes under stress; the compliance values and long-range bending proxy are resolution dependent.

### Bending and membrane models

Long-range distance bending is cheap and nonzero in a flat rest state, but couples curvature to in-plane stretch and depends strongly on tessellation. Dihedral-angle constraints map intuitively to fold angle but have a poorly conditioned gradient around exactly flat configurations. The isometric bending energy from discrete shells/PBD uses the two triangles adjacent to an interior edge and a cotangent-derived quadratic form. It better separates bending from membrane stretch, costs four-particle gradients, and still inherits mesh/material scaling concerns.

Triangle strain constraints (Müller et al., 2014) operate on deformation gradients and naturally separate warp, weft, and shear. Corotational/StVK/Neo-Hookean membrane models provide more physical constitutive behavior but require careful inversion handling and calibration. For a flag whose artistic presets are not laboratory fitted, adding parameters is only useful if the visible result justifies the added tuning surface. The present distance network already prevents gross stretch efficiently.

### Projective Dynamics

Projective Dynamics (Bouaziz et al., 2014) alternates local projections with a global sparse linear solve. With fixed topology and stiffness, the system matrix can be prefactorized, making stiff deformation efficient and predictable. Pins, time-varying material/quality rebuilds, and interactive grab constraints complicate—but do not preclude—the factorization. A browser implementation would need a maintained sparse factorization or iterative global solver and a second material/aero integration path. Domain-decomposed PD (Lu et al., 2025) is impressive for multicore high-resolution CPU cloth, but its payoff depends on domain parallelism absent from this single-thread WASM deployment. A deliberately incomplete PD toy would not be a credible comparison, so it was researched but not implemented.

### FEM, discrete shells, IPC, and C-IPC

Discrete shells and nonlinear shell FEM model membrane and curvature energy more directly. Robust production use generally requires implicit integration, sparse nonlinear solves, line search, and careful inversion/contact treatment. IPC adds barrier potentials, continuous collision detection, and nonlinear optimization to guarantee intersection-free trajectories under its assumptions. C-IPC extends those ideas to finite-thickness codimensional shells/rods, friction, strain barriers, and additive CCD. These methods are appropriate when guaranteed contact and engineering/offline fidelity dominate; they are disproportionate for a freely hanging browser flag with 1,600–6,400 vertices and immediate input response.

Progressive Dynamics++ (Zhang et al., 2025) maintains consistent animation while progressively refining space and time. It informs future adaptive-quality research, but the current quality selector reconstructs a simulation explicitly and does not need continuous multiresolution correspondence.

## Contact research

Real cloth self-contact requires broadphase pruning, topological exclusions, vertex–face and edge–edge narrow phases, finite thickness, stable response, and ideally continuous handling to prevent tunneling. Uniform spatial hashes are simple and WASM-friendly; BVHs improve sparse/deformed workloads but require refit/rebuild logic. XPBD repulsion constraints are plausible for an interactive approximation but do not guarantee intersection-free paths.

IPC/C-IPC provide strong guarantees at the cost of CCD and nonlinear solves. Offset Geometric Contact (Chen et al., 2025) uses offset geometry and displacement bounds to reduce expensive CCD and expose local GPU-parallel work. Borrowing finite offset thickness is sensible, but calling a vertex repulsion scheme “OGC” would be inaccurate. Production self-collision was not selected: a top-edge flag rarely benefits enough to justify its unpredictable pair count, extra latency, and tuning risk. See [CONTACT_RESEARCH.md](CONTACT_RESEARCH.md).

## Aerodynamics and flutter

The local velocity seen by a triangle is `v_air - v_surface`, not merely `v_air`. Pressure depends on signed normal incidence and projected area; tangential skin drag acts on the component parallel to the surface. The implemented candidate averages the three vertex velocities, subtracts that from air velocity, and applies the existing two-sided quadratic pressure/drag law. This introduces aerodynamic damping naturally: a triangle moving with the air sees less load, while a triangle moving against it sees more.

Real flag flutter is a fluid–structure instability involving separated flow and vortex shedding. A triangle-drag model cannot reproduce a wake-resolving FSI solution. Convincing interactive flutter instead emerges from relative velocity, changing normals, inertia, flexible bending, and irregular ambient/gust fields. Prescribed mesh sinusoids were rejected.

## Browser execution choices

- **Scalar WASM CPU:** broad compatibility, low input latency, deterministic core tests, no transfer/readback, and adequate cost at current resolutions.
- **SIMD128:** useful only after a data-layout/profile finding. The present AoS constraint loop has irregular indexed gathers; converting it to SoA purely to expose SIMD is not justified without a measured win. Normal/integration kernels are candidates for future focused work.
- **WASM threads:** SharedArrayBuffer requires cross-origin isolation headers. Ordinary GitHub Pages/custom-domain hosting does not provide a convenient required-header contract, and graph/domain scheduling would add complexity. Threads therefore remain optional research, never a requirement.
- **WebGPU compute:** major current desktop browsers increasingly expose WebGPU, and compute can scale Jacobi/color-batched constraints and collision broadphases. It also means a new solver, synchronization model, shader tests, fallback, and potentially delayed CPU observability. At 6,400 vertices the measured CPU solver still offers direct next-step response. WebGL2 rendering plus CPU/WASM physics remains the practical production choice.

## Initial hypothesis and final outcome

The initial hypothesis was that true XPBD plus Small Steps and isometric bending could reduce stretch and improve resolution behavior. The experiments only partly support it: default-mesh stretch improved, but direct Small Steps was unstable at higher resolution, true XPBD made the existing compliance presets softer, and isometric bending added substantial cost/tuning risk. The reliable improvement was aerodynamic relative velocity plus removal of an avoidable index-buffer clone. The final evidence and decision matrix are in [BENCHMARK_RESULTS.md](BENCHMARK_RESULTS.md) and [DECISION.md](DECISION.md).
